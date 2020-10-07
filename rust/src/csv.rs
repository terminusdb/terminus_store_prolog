use chardetng::*;
use csv::ReaderBuilder;
use encoding_rs::UTF_8;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use terminus_store::layer::StringTriple;
use terminus_store::store::sync::*;
use hex;
use sha1::{Sha1, Digest};
use urlencoding;

fn check_utf8(csv_path: PathBuf) -> bool {
    let csv_path_clone = csv_path.clone();
    let mut f = File::open(csv_path_clone).unwrap();
    let mut buffer = [0; 2048];
    // read a chunk
    let _n = f.read(&mut buffer);

    let mut enc_detector = EncodingDetector::new();

    enc_detector.feed(&buffer, true);

    let res = enc_detector.guess(None, true);

    if res == UTF_8 {
        return true;
    } else {
        return false;
    }
}

pub fn import_csv(
    csv_name: String,
    csv_path: String,
    builder: &SyncStoreLayerBuilder,
    data_prefix: String,
    schema_prefix: String,
    has_header: bool,
    skip_header: bool,
) -> std::result::Result<(), io::Error> {
    let pathbuf: PathBuf = csv_path.into();

    if !check_utf8(pathbuf.clone()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not convert to utf-8",
        ));
    }
    let file = File::open(pathbuf)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(has_header && !skip_header)
        .from_reader(file);

    let mut header = Vec::new();
    let mut column_names = Vec::new();
    if !has_header || skip_header {
        let len = reader.headers().unwrap().len();
        for i in 0..len {
            header.push(format!("{}col{}", schema_prefix, i));
            column_names.push(format!("{}", i));
        }
    } else {
        for field in reader.headers().unwrap().iter() {
            let escaped_field = urlencoding::encode(field);
            column_names.push(String::from(field));
            header.push(format!("{}{}", schema_prefix, escaped_field));
        }
    }

    let rdf_type = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let label = "http://www.w3.org/2000/01/rdf-schema#label";
    // Create the csv type
    let csv_type = format!("{}{}", schema_prefix, "Csv");
    let csv_name_escaped = urlencoding::encode(&csv_name);
    let csv_name_value = format!("{:?}^^'http://www.w3.org/2001/XMLSchema#string'", csv_name);
    let csv_node = format!("{}{}", data_prefix, csv_name_escaped);
    builder.add_string_triple(StringTriple::new_node(&csv_node,
                                                     &rdf_type,
                                                     &csv_type))
        .unwrap();
    builder.add_string_triple(StringTriple::new_value(&csv_node,
                                                      &label,
                                                      &csv_name_value))
        .unwrap();
    // Create the ordered column names metadata for the csv
    let mut column_index = 0;
    for field in column_names.iter() {
        let escaped_field = urlencoding::encode(field);
        let column_predicate = "csv:///schema#column";
        let column_node = format!("csv:///data/ColumnObject_{}_{}", csv_name_escaped, escaped_field);
        let column_type = "csv:///schema#Column";
        let column_index_predicate = "csv:///schema#index";
        let column_index_value = format!("{}^^'http://www.w3.org/2001/XMLSchema#integer'", column_index);
        let column_name_predicate = "csv:///schema#column_name";
        let column_name_value = format!("{:?}^^'http://www.w3.org/2001/XMLSchema#string'", field);
        builder.add_string_triple(StringTriple::new_node(&csv_node,
                                                         &column_predicate,
                                                         &column_node))
            .unwrap();
        builder.add_string_triple(StringTriple::new_node(&column_node,
                                                         &rdf_type,
                                                         &column_type))
            .unwrap();
        builder.add_string_triple(StringTriple::new_value(&column_node,
                                                          &column_index_predicate,
                                                          &column_index_value))
            .unwrap();
        builder.add_string_triple(StringTriple::new_value(&column_node,
                                                          &column_name_predicate,
                                                          &column_name_value))
            .unwrap();
        column_index += 1;
    }

    // Create a unique Row type based on ordered column names
    let mut column_hasher = Sha1::new();
    let mut sorted_column_names = column_names.clone();
    sorted_column_names.sort();
    for field in sorted_column_names.iter() {
        // create a Sha1 object
        column_hasher.update(field);
    }
    let column_hash = column_hasher.finalize();
    let column_hash_string = hex::encode(column_hash);

    let row_type = format!("{}{}_{}", schema_prefix, "Row", column_hash_string);
    reader
        .into_records()
        .enumerate()
        .for_each(|(_line, record)| {
            let record = record.unwrap();

            // create a Sha1 object
            let mut hasher = Sha1::new();

            // process input message
            for (_, field) in record.iter().enumerate() {
                hasher.update(field);
            }
            let hash = hasher.finalize();
            let hash_string = hex::encode(hash);
            let node = format!("{}row{}", data_prefix, hash_string);
            let row_predicate = format!("{}{}", schema_prefix, "row");

            // add row type
            builder
                .add_string_triple(StringTriple::new_node(&node, &rdf_type, &row_type))
                .unwrap();
            // add row predicate
            builder
                .add_string_triple(StringTriple::new_node(&csv_node, &row_predicate, &node))
                .unwrap();
            for (col, field) in record.iter().enumerate() {
                let value = format!("{:?}^^'http://www.w3.org/2001/XMLSchema#string'", field);
                let column = &header[col];
                builder
                    .add_string_triple(StringTriple::new_value(&node, &column, &value))
                    .unwrap();
            }
        });

    return Ok(());
}
