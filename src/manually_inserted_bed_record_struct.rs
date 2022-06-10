/// This message represents a single BED record. See
/// <https://genome.ucsc.edu/FAQ/FAQformat.html#format1> for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BedRecord {
    /// The reference on which this variant occurs. Corresponds to "CHROM" in UCSC.
    #[prost(string, tag = "1")]
    pub reference_name: ::prost::alloc::string::String,
    /// The position at which this region occurs (0-based inclusive).
    #[prost(int64, tag = "2")]
    pub start: i64,
    /// The position at which this region ends (0-based exclusive).
    #[prost(int64, tag = "3")]
    pub end: i64,
    /// The name of the record.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// As described in <https://genome.ucsc.edu/FAQ/FAQformat.html#format1,> score
    /// should be an integer in [0, 1000]. However, many non-integer values are
    /// seen in BED records in the wild, so we represent this as a double.
    #[prost(double, tag = "5")]
    pub score: f64,
    /// The strand on the genome that the record is on.
    #[prost(enumeration = "manually_inserted_bed_record_struct::Strand", tag = "6")]
    pub strand: i32,
    /// For visualization purposes, the position at which the feature starts to be
    /// drawn thickly. In gene structures this corresponds to the start codon.
    /// This is zero-based inclusive numbering, like the `start` field.
    #[prost(int64, tag = "7")]
    pub thick_start: i64,
    /// For visualization purposes, the position at which the feature stops being
    /// drawn thickly. In gene structures this corresponds to the stop codon.
    /// This is zero-based exclusive numbering, like the `end` field.
    #[prost(int64, tag = "8")]
    pub thick_end: i64,
    /// A comma-separated RGB value R,G,B for visualization.
    #[prost(string, tag = "9")]
    pub item_rgb: ::prost::alloc::string::String,
    /// The number of distinct blocks in the BED line (e.g. exon count in gene
    /// structures).
    #[prost(int32, tag = "10")]
    pub block_count: i32,
    /// Comma-separated list of block sizes. The number of items in the list should
    /// be equal to `block_count`.
    #[prost(string, tag = "11")]
    pub block_sizes: ::prost::alloc::string::String,
    /// Comma-separated list of block start positions. The number of items in the
    /// list should be equal to `block_count`.
    /// This is zero-based inclusive numbering, like the `start` field.
    #[prost(string, tag = "12")]
    pub block_starts: ::prost::alloc::string::String,
}
/// Nested message and enum types in `BedRecord`.
pub mod manually_inserted_bed_record_struct {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Strand {
        /// The strand is unspecified, unknown, or not meaningful.
        NoStrand = 0,
        ForwardStrand = 1,
        ReverseStrand = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BedHeader {
    /// The number of fields in the BED file.
    #[prost(int32, tag = "1")]
    pub num_fields: i32,
}
///////////////////////////////////////////////////////////////////////////////
// I/O-related messages.
///////////////////////////////////////////////////////////////////////////////

/// Options for reading BED files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BedReaderOptions {
    /// Optional. The number of fields to read from the BED file. If this is unset,
    /// or set to more fields than are present in the BED file, all fields are
    /// read.
    #[prost(int32, tag = "2")]
    pub num_fields: i32,
}
/// Options for writing BED files.
/// Currently this is a placeholder message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BedWriterOptions {}
