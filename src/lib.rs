pub mod error;

//use async_trait::async_trait;
use serde::{Serializer, ser::SerializeStruct};
//use serde::{Deserializer, ser::DeserializeStruct};
//use error::BioSerdeError;
//use arrow::record_batch::{RecordBatchReader};

// TODO: This should ideally be moved to a specific module (crate?) encapsulating 
// all traditional bioinformatics formats and its associated (util) methods.
// use noodles::bam::AsyncReader as BamReader;
// use noodles::bcf::AsyncReader as BcfReader;
//use noodles::bed::Reader as BedReader;
use noodles::bed::Writer as BedWriter;
// use noodles::cram::AsyncReader as CramReader;
// use noodles::fasta::AsyncReader as FastaReader;
// use noodles::fastq::AsyncReader as FastqReader;
// use noodles::vcf::AsyncReader as VcfReader;

// Refer to https://serde.rs/custom-serialization.html for details on
// Custom Serialization.
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
struct ModernBioFormat<F> {
    writer: F,
    destination: String,
}

impl Serialize for ModernBioFormat<F: BedWriter> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bio = serializer.serialize_struct("ModernBioFormat", 2)?;
        bio.serialize_field("format", &self.format)?;
        bio.serialize_field("destination", &self.destination)?;
        bio.end()
    }
}

// impl Deserialize for TraditionalBioFormat {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer,
//     {
//         deserializer.deserialize_struct("Color", &["r", "g", "b"], ColorVisitor)
//     }
// }