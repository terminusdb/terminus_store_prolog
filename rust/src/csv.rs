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
    if !has_header || skip_header {
        let len = reader.headers().unwrap().len();
        for i in 0..len {
            header.push(format!("{}col{}", schema_prefix, i));
        }
    } else {
        for field in reader.headers().unwrap().iter() {
            let escaped_field = urlencoding::encode(field);
            header.push(format!("{}{}", schema_prefix, escaped_field));
        }
    }

    let rdf_type = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let row_type = format!("{}{}", schema_prefix, "Row");

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
            // add row type
            builder
                .add_string_triple(StringTriple::new_node(&node, &rdf_type, &row_type))
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
