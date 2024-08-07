extern crate flate2;

use ::flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    // check if the command line argument is greater then three
    if args().len() != 3 {
        eprintln!("Usage `source` `target`");
        return;
    }

    let source_path = args().nth(1).unwrap();
    let target_path = args().nth(2).unwrap();

    // let read from the Buffer
    let input_file = File::open(&source_path).unwrap();

    let mut input = BufReader::new(input_file);
    // out the file and the path
    let output = File::create(&target_path).unwrap();
    // the encoder
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    let duration = start.elapsed();
    // finilize the encoder to make sure all data are written
    encoder.finish().unwrap();

    let source_len = std::fs::metadata(&source_path).unwrap().len();
    let target_len = std::fs::metadata(&target_path).unwrap().len();

    println!("Completed the compression {:?}", duration);
    println!("The source file size {:?}", source_len);
    println!("The target file size {:?}", target_len);
}
