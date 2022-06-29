use thiserror::Error;

#[derive(Error, Debug)]
pub enum BioSerdeError {
    // Enumerate and wrap the underlying Arrow error(s), for instance
}