use std::io;
use thiserror;


// A Trait by which files should adhere


pub trait BioFormat {
    fn load_whole(&self, path: &std::path::Path) -> Result<(), FileError>;
    fn load_header(&self, path: &std::path::Path) -> Result<(), FileError>;

    fn save(&self, path: &std::path::Path) -> Result<(), FileError>;
}


#[derive(thiserror::Error, Debug)]
pub enum FileError {
    #[error("Cannot open file!")]
    FileOpen(#[source] io::Error),
    #[error("Cannot save to file!")]
    FileSave(#[source] io::Error),
    #[error("Failed to parse record!")]
    RecordRead(#[source] io::Error),
    #[error("Failed to write! record!")]
    RecordWrite(#[source] io::Error),
}

