//! Showcases struct (de)serialization for BED

use std::{fs::{File, self}, io::BufReader};

use noodles_bed as bed;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = "../data/bed/well_formed_sample.bed";

    let mut reader = File::open(src).map(BufReader::new).map(bed::Reader::new)?;
    let reference = fs::read_to_string(src)?;
    let serialized = bed::to_string(&reader)?;

    assert_eq!(reference, serialized);

    Ok(())
}