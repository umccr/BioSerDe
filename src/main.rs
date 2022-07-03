//! Showcases struct (de)serialization for BED

use std::{fs::{File, self}, io::BufReader};

use bed::Record;
use noodles_bed as bed;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = "data/bed/well_formed_sample.bed";

    let mut reader = File::open(src).map(BufReader::new).map(bed::Reader::new)?;
    let reference = fs::read_to_string(src)?;

    println!("{:?}", reference);

    let bed_records: Vec<Record<3>> = bed::from_reader(&mut reader);
    println!("{:?}", bed_records);

    let bed_record_serialization= bed::to_string(&bed_records)?;
    println!("{:#?}", bed_record_serialization);

    // assert_eq!(reference, serialized);

    Ok(())
}