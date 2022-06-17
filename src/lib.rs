pub mod error;

use serde::{Serializer, Deserializer};
use error::BioSerdeError;
use arrow::record_batch::{RecordBatchReader};
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, BioSerdeError>
    where
        S: Serializer;
}

// Refer to https://serde.rs/custom-serialization.html for details on
// Custom Serialization.
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, BioSerdeError>
    where
        D: Deserializer<'de>;
}

// This lets us re-use the same code for parquet and feather
enum SerializationFormat {
    Parquet,
    Feather,
    ORC,
}
// Technically this is our own IR, but it's just a very light wrapper around RecordBatch
pub struct IR<T> {
    ir: T,
    target_format: SerializationFormat
}

/// Traditional Bioinformatics format SerDe
pub trait BioFormat<B> where B: RecordBatchReader {
    fn deserialize(&mut self, data: &[u8]) -> Result<(), BioSerdeError>;
    fn serialize(&self) -> Result<IR<B>, BioSerdeError>;
}

/// Modern cloud native format SerDe
pub trait TargetFormat<T> where T: RecordBatchReader {
    fn deserialize(&mut self, data: &[u8]) -> Result<(), BioSerdeError>;
    fn serialize(&self) -> Result<IR<T>, BioSerdeError>;
}