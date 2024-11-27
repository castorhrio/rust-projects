use clap::{Arg, Command};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    let matches = Command::new("Rust GZip")
        .version("1.0")
        .author("Author By Xyris")
        .about("GZip compression tool")
        .arg(
            Arg::new("source")
                .help("sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("target")
                .help("sets the outpot file to use,recommonded with `.gz` extension")
                .required(true)
                .index(2),
        )
        .get_matches();

    let source = matches.get_one::<String>("source").unwrap();
    let target = matches.get_one::<String>("target").unwrap();

    let input = File::open(source).expect("Failed to open file");
    let mut input = BufReader::new(input);

    let output = File::create(target).expect("Failed to create file");
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).expect("Failed to compress file");
    encoder.finish().expect("Failed to finish compression");

    println!("Compression took:{:?}", start.elapsed());

    println!(
        "Original file size:{}",
        input.get_ref().metadata().unwrap().len()
    );
}
