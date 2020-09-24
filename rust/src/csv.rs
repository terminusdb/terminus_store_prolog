use chardetng::*;
use csv::ReaderBuilder;
use encoding_rs::UTF_8;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use terminus_store::layer::StringTriple;
use terminus_store::store::sync::*;

fn normalize_header(header: &str) -> String {
    header.to_owned()
}

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
    predicate_prefix: String,
    has_header: bool,
    skip_header: bool,
) -> std::result::Result<SyncStoreLayer, io::Error> {
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
            header.push(format!("{}col{}", predicate_prefix, i));
        }
    } else {
        for field in reader.headers().unwrap().iter() {
            header.push(format!("{}{}", predicate_prefix, normalize_header(field)));
        }
    }

    reader
        .into_records()
        .enumerate()
        .for_each(|(line, record)| {
            let record = record.unwrap();
            let node = format!("{}row{}", data_prefix, line);
            for (col, field) in record.iter().enumerate() {
                let value = format!("{:?}^^'http://www.w3.org/2001/XMLSchema#string'", field);
                builder
                    .add_string_triple(StringTriple::new_value(&node, &header[col], &value))
                    .unwrap();
            }
        });

    let layer = builder.commit()?;
    return Ok(layer);
}
