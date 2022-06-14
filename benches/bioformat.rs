// A Trait by which benchable forms should adhere

// TODO: Example of how to handle a format that doesn't impl some trait fn.
// TODO: Such as only some formats having headers.

pub trait BioFormat {
    fn read_whole(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn read_header(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn write(&self) -> Result<(), Box<dyn std::error::Error>>;
}
