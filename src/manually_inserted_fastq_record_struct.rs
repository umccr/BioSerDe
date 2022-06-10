use serde::{Deserialize, Serialize};

pub fn test_serde_fastq() {
    let fastq = FastqRecord {
        id: "".to_string(),
        description: "".to_string(),
        sequence: "".to_string(),
        quality: "".to_string(),
    };

    println!("{:?}", fastq);

    let serialized = serde_json::to_string(&fastq).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: FastqRecord = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

/// This message represents a single FASTQ record.
#[derive(Serialize, Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FastqRecord {
    /// The first line of a FASTQ record begins with '@' and is followed by a
    /// sequence identifier (up to the first whitespace character) and then an
    /// optional description. This line is parsed into its constituent id and
    /// description.
    /// The sequence identifier.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. The description provided in the header line.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The raw sequence letters.
    #[prost(string, tag = "3")]
    pub sequence: ::prost::alloc::string::String,
    /// The quality values for the sequence. Its length must be the same as the
    /// sequence length, and is encoded in ASCII. The meaning of each base quality
    /// may vary: it is usually a Phred-scaled score
    /// (-10 * log_10(Pr{call is incorrect})) but differs for some older versions
    /// of FASTQs.
    #[prost(string, tag = "4")]
    pub quality: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FastqReaderOptions {
    /// If true, simply drop invalid records. Otherwise, raise an error on invalid
    /// records.
    #[prost(bool, tag = "2")]
    pub skip_invalid_records: bool,
}
/// Options for writing FASTQ files.
/// Currently this is a placeholder message but could be used to support
/// different choices on output like whether the pad line should include the
/// header or not.
#[derive(Serialize, Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FastqWriterOptions {}
