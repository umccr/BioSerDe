// Generated from bed.proto
pub mod bed {
    include!(concat!(env!("OUT_DIR"), "/nucleus.genomics.v1.rs"));
}

pub mod manually_inserted_bed_record_struct;

pub fn create_simple_bedfile(_refname: &str) -> bed::BedRecord {
    let bed = bed::BedRecord::default();
    bed
}

pub fn create_manual_bedfile(_refname: &str) -> manually_inserted_bed_record_struct::BedRecord {
    let bed = manually_inserted_bed_record_struct::BedRecord::default();
    bed
}
