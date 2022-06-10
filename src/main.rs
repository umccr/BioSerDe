use bioserde::{
    manually_inserted_bed_record_struct::test_serde_bed,
    manually_inserted_fastq_record_struct::test_serde_fastq,
};

fn main() {
    dbg!(test_serde_bed());
    dbg!(test_serde_fastq());
}
