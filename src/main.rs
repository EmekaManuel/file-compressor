extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    if args.len() != 3 {
        eprintln!("Usage: <source> <target>");
        return;
    }

    // Extract arguments
    let source = &args[1];
    let target = &args[2];

    // Open the source file and create the target file
    let input_file = File::open(source).expect("Failed to open source file");
    let mut input = BufReader::new(input_file);
    let output_file = File::create(target).expect("Failed to create target file");

    // Set up the GzEncoder
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // Measure compression time
    let start = Instant::now();
    copy(&mut input, &mut encoder).expect("Failed to compress data");
    let output = encoder.finish().expect("Failed to finalize compression");

    // Print statistics
    println!(
        "Source length: {} bytes",
        input
            .get_ref()
            .metadata()
            .expect("Failed to get source metadata")
            .len()
    );
    println!(
        "Target length: {} bytes",
        output
            .metadata()
            .expect("Failed to get target metadata")
            .len()
    );
    println!("Elapsed time: {:?}", start.elapsed());
}
