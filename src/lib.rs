// Generated from bed.proto
pub mod bed {
    include!(concat!(env!("OUT_DIR"), "/nucleus.genomics.v1.rs"));
}

pub fn create_simple_bedfile(_refname: &str) -> bed::BedRecord {
    let bed = bed::BedRecord::default();
    bed
}