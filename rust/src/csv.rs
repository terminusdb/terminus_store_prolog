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
    schema_builder_option: Option<&SyncStoreLayerBuilder>,
    data_prefix: String,
    schema_prefix: String,
    has_header: bool,
    skip_header: bool,
) -> Result<(), csv::Error> {
    let pathbuf: PathBuf = csv_path.into();

    if !check_utf8(pathbuf.clone()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not convert to utf-8",
        ).into());
    }
    let file = File::open(pathbuf)?;

    let mut reader = ReaderBuilder::new()
        .has_headers(has_header && !skip_header)
        .from_reader(file);

    let mut header = Vec::new();
    let mut column_names = Vec::new();

    let headers = reader.headers();
    // if headers.is_error(){
    //     return io::Error::new(io::ErrorKind::UnexpectedEof,"There are no lines in this CSV").into();
    // }

    if !has_header || skip_header {
        let len = headers.expect("Expected a Some for headers but headers are empty").len();
        for i in 0..len {
            header.push(format!("{}column_{}", schema_prefix, i));
            column_names.push(format!("{}", i));
        }
    } else {
        for field in headers.expect("Expected a Some for headers but headers are empty").iter() {
            let escaped_field = urlencoding::encode(field);
            column_names.push(String::from(field));
            header.push(format!("{}column_{}", schema_prefix, escaped_field));
        }
    }

    // Prefixes
    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
    let rdfs = "http://www.w3.org/2000/01/rdf-schema#";
    let xsd = "http://www.w3.org/2001/XMLSchema#";

    let rdf_type = format!("{}{}", rdf, "type");
    let label = format!("{}{}", rdfs, "label");

    // Create the csv type
    let csv_type = format!("{}CSV", schema_prefix);
    let csv_name_escaped = urlencoding::encode(&csv_name);
    let csv_name_value = format!("{:?}@en", csv_name);
    let csv_node = format!("{}CSV_{}", data_prefix, csv_name_escaped);
    builder.add_string_triple(StringTriple::new_node(&csv_node,
                                                     &rdf_type,
                                                     &csv_type))?;
    builder.add_string_triple(StringTriple::new_value(&csv_node,
                                                      &label,
                                                      &csv_name_value))?;

    // Create the ordered column names metadata for the csv
    let mut column_index = 0;
    for field in column_names.iter() {
        let escaped_field = urlencoding::encode(field);
        let column_predicate = format!("{}csv_column", schema_prefix);
        let column_node = format!("{}ColumnObject_{}_{}", data_prefix, csv_name_escaped, escaped_field);
        let column_type = format!("{}Column", schema_prefix);
        let column_index_predicate = format!("{}csv_column_index", schema_prefix);
        let column_index_value = format!("{}^^'{}{}'", column_index, xsd, "integer");
        let column_name_predicate = format!("{}csv_column_name", schema_prefix);
        let column_name_value = format!("{:?}^^'{}{}'", field, xsd, "string");

        builder.add_string_triple(StringTriple::new_node(&csv_node,
                                                         &column_predicate,
                                                         &column_node))?;

        builder.add_string_triple(StringTriple::new_node(&column_node,
                                                         &rdf_type,
                                                         &column_type))?;

        builder.add_string_triple(StringTriple::new_value(&column_node,
                                                          &column_index_predicate,
                                                          &column_index_value))?;

        builder.add_string_triple(StringTriple::new_value(&column_node,
                                                          &column_name_predicate,
                                                          &column_name_value))?;
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

    if let Some(schema_builder) = schema_builder_option {
        write_schema(schema_builder, &schema_prefix, &column_hash_string, &sorted_column_names)?;
    }

    for result in reader.records() {
        let record = result?;

        // create a Sha1 object
        let mut hasher = Sha1::new();

        // process input message
        for (_, field) in record.iter().enumerate() {
            hasher.update(field);
        }
        let hash = hasher.finalize();
        let hash_string = hex::encode(hash);
        let node = format!("{}CSVRow_{}", data_prefix, hash_string);
        let row_predicate = format!("{}csv_row", schema_prefix);
        let row_type = format!("{}CSVRow_{}", schema_prefix, column_hash_string);

        // add row type
        builder
            .add_string_triple(StringTriple::new_node(&node, &rdf_type, &row_type))?;
        // add row predicate
        builder
            .add_string_triple(StringTriple::new_node(&csv_node, &row_predicate, &node))?;
        for (col, field) in record.iter().enumerate() {
            let value = format!("{:?}^^'{}{}'", field, xsd, "string");
            let column = &header[col];
            builder
                .add_string_triple(StringTriple::new_value(&node, &column, &value))?;
        }
        return Ok(())
    };

    return Ok(());
}

fn write_schema(schema_builder: &SyncStoreLayerBuilder,
                schema_prefix: &str,
                column_hash_string: &str,
                sorted_column_names: &Vec<String>) -> Result<(), csv::Error>{
    // Prefixes
    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
    let rdfs = "http://www.w3.org/2000/01/rdf-schema#";
    let xsd = "http://www.w3.org/2001/XMLSchema#";
    let owl = "http://www.w3.org/2002/07/owl#";

    // common predicates
    let rdf_type = format!("{}{}", rdf, "type");
    let label = format!("{}{}", rdfs, "label");
    let comment = format!("{}{}", rdfs, "comment");
    let domain = format!("{}{}", rdfs, "domain");
    let range = format!("{}{}", rdfs, "range");
    let owl_class = format!("{}{}", owl, "Class");
    let datatype_property = format!("{}{}", owl, "DatatypeProperty");
    let object_property = format!("{}{}", owl, "ObjectProperty");
    let sub_class_of = format!("{}{}", rdfs, "subClassOf");

    // common types
    let xsd_string = format!("{}{}", xsd, "string");
    let xsd_integer = format!("{}{}", xsd, "integer");

    // Create the csv object
    let csv_type = format!("{}CSV", schema_prefix);
    let csv_label = "\"CSV\"@en";
    let csv_comment = "\"CSV object\"@en";
    let document = "http://terminusdb.com/schema/system#Document";

    schema_builder
        .add_string_triple(StringTriple::new_node(&csv_type, &rdf_type, &owl_class))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&csv_type, &label, &csv_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&csv_type, &comment, &csv_comment))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&csv_type, &sub_class_of, &document))?;

    // Create column objects and fields
    let column_predicate = format!("{}csv_column", schema_prefix);
    let column_predicate_label = "\"csv column\"@en";
    let column_predicate_comment = "\"Associates a CSV with a column object\"@en";

    let column_type = format!("{}Column", schema_prefix);
    let column_label = "\"Column\"@en";
    let column_comment = "\"Column information object for a CSV\"@en";
    let column_index_predicate = format!("{}csv_column_index", schema_prefix);
    let column_index_label = "\"csv column index\"@en";
    let column_index_comment = "\"The ordering index for a column in a csv\"@en";
    let column_name_predicate = format!("{}csv_column_name", schema_prefix);
    let column_name_label = "\"csv column index\"@en";
    let column_name_comment = "\"The name of the column as it was verbatim in the CSV\"@en";

    schema_builder
        .add_string_triple(StringTriple::new_node(&column_type, &rdf_type, &owl_class))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_type, &label, &column_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_type, &comment, &column_comment))?;
    // column
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_predicate, &rdf_type, &object_property))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_predicate, &label, &column_predicate_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_predicate, &comment, &column_predicate_comment))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_predicate, &domain, &csv_type))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_predicate, &range, &column_type))?;

    // index
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_index_predicate, &rdf_type, &datatype_property))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_index_predicate, &label, &column_index_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_index_predicate, &comment, &column_index_comment))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_index_predicate, &domain, &column_type))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_index_predicate, &range, &xsd_integer))?;

    // name
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_name_predicate, &rdf_type, &datatype_property))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_name_predicate, &label, &column_name_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&column_name_predicate, &comment, &column_name_comment))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_name_predicate, &domain, &column_type))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&column_name_predicate, &range, &xsd_string))?;

    // Row super class
    let row_super = format!("{}CSVRow", schema_prefix);
    let row_super_label = "\"CSV Row\"@en";
    let row_super_comment = "\"Generic Row of a CSV file\"@en";

    schema_builder
        .add_string_triple(StringTriple::new_node(&row_super, &rdf_type, &owl_class))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&row_super, &label, &row_super_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&row_super, &comment, &row_super_comment))?;

    // Create Row types
    let row_type = format!("{}CSVRow_{}", schema_prefix, column_hash_string);
    let row_label = format!("\"CSV Row {}\"@en", column_hash_string);
    let sorted_column_string = format!("CSV Row object for columns {:?}", sorted_column_names);
    let row_comment = format!("{:?}@en", sorted_column_string);

    schema_builder
        .add_string_triple(StringTriple::new_node(&row_type, &rdf_type, &owl_class))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&row_type, &sub_class_of, &row_super))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&row_type, &label, &row_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&row_type, &comment, &row_comment))?;

    // Row predicate
    let row_predicate = format!("{}csv_row", schema_prefix);
    let row_predicate_label = "\"csv row\"@en";
    let row_predicate_comment = "\"Connects a CSV to its rows\"@en";

    schema_builder
        .add_string_triple(StringTriple::new_node(&row_predicate, &rdf_type, &object_property))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&row_predicate, &label, &row_predicate_label))?;
    schema_builder
        .add_string_triple(StringTriple::new_value(&row_predicate, &comment, &row_predicate_comment))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&row_predicate, &domain, &csv_type))?;
    schema_builder
        .add_string_triple(StringTriple::new_node(&row_predicate, &range, &row_type))?;

    // Create column predicates for each field
    for field in sorted_column_names {
        let escaped_field = urlencoding::encode(field);
        let column_p = format!("{}column_{}", schema_prefix, escaped_field);
        let column_label = format!("\"Column {}\"@en", field);
        let column_comment = format!("\"CSV Column for header name {}\"@en", field);

        schema_builder
            .add_string_triple(StringTriple::new_node(&column_p, &rdf_type, &datatype_property))?;
        schema_builder
            .add_string_triple(StringTriple::new_value(&column_p, &label, &column_label))?;
        schema_builder
            .add_string_triple(StringTriple::new_value(&column_p, &comment, &column_comment))?;
        schema_builder
            .add_string_triple(StringTriple::new_node(&column_p, &domain, &row_type))?;
        schema_builder
            .add_string_triple(StringTriple::new_node(&column_p, &range, &xsd_string))?;
    }
    return Ok(())
}
