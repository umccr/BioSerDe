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
    println!("{:?}", bed_record_serialization);

    let test_1 = r#"{"chrom":"chr7","start":127471197,"end":127472363}"#;
    let bed_record_deserialization_test_1: Record::<3> = bed::from_str(test_1).unwrap();
    println!("{:?}", bed_record_deserialization_test_1);

    // // Doesn't work yet
    // // (Should it?)
    // let test_2 = r#"[
    //     {"chrom":"chr7","start":127471197,"end":127472363},
    //     {"chrom":"chr7","start":127472364,"end":127473530},
    //     {"chrom":"chr7","start":127473531,"end":127474697}
    // ]"#;
    // let bed_record_deserialization_test_2: Vec<Record::<3>> = bed::from_str(test_2).unwrap();
    // println!("{:?}", bed_record_deserialization_test_2);

    // assert_eq!(reference, serialized);

    Ok(())
}