pub mod error;

use async_trait::async_trait;
use serde::{Serializer, Deserializer};
use error::BioSerdeError;
//use arrow::record_batch::{RecordBatchReader};

// TODO: This should ideally be moved to a specific module (crate?) encapsulating 
// all traditional bioinformatics formats and its associated (util) methods.
// use noodles::bam::AsyncReader as BamReader;
// use noodles::bcf::AsyncReader as BcfReader;
// use noodles::bed::Reader as BedReader;
// use noodles::cram::AsyncReader as CramReader;
// use noodles::fasta::AsyncReader as FastaReader;
// use noodles::fastq::AsyncReader as FastqReader;
// use noodles::vcf::AsyncReader as VcfReader;

// Noodles Readers for evey traditional format
pub enum NoodlesReader {
    BamReader,
    BcfReader,
    BedReader,
    CramReader,
    FastaReader,
    FastqReader,
    VcfReader,
}

// Refer to https://serde.rs/custom-serialization.html for details on
// Custom Serialization.
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, BioSerdeError>
    where
        S: Serializer;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, BioSerdeError>
    where
        D: Deserializer<'de>;
}

#[allow(dead_code)]
// This lets us re-use the same code for parquet and feather
enum SerializationFormat {
    Parquet,
    Feather,
    ORC,
}

#[allow(dead_code)]
// This is our own (swappable) Internal Representation
pub struct IR<T> {
    ir: T,
    target_format: SerializationFormat
}

/// 
#[async_trait]
pub trait TraditionalBioFormatReader<F> {
    async fn get_reader(self, source: &str, format: NoodlesReader) -> Result<F, BioSerdeError>;
}

/// Traditional Bioinformatics format SerDe
#[async_trait]
pub trait TraditionalBioFormatDeSerialize<F, I> {
    /// Deserialize traditional bioinformatics format into an arbitrary Internal Representation
    async fn deserialize(&self, reader: dyn TraditionalBioFormatReader<F>) -> Result<IR<I>, BioSerdeError>;
    //fn serialize(&self) -> Result<IR<B>, BioSerdeError>; <-- we will not write back traditional formats
}

/// Modern cloud native format SerDe
#[async_trait]
pub trait ModernBioFormatSerialize<B, T> {
    //fn deserialize(&mut self, data: &[u8]) -> Result<(), BioSerdeError>; <-- one way: forward conversion only
    /// Serialize internal representation into destination format
    async fn serialize(&self, internal: IR<B>, destination: &str) -> Result<IR<T>, BioSerdeError>;
}