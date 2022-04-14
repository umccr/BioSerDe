use std::io::Result;
use prost_build;

fn main() -> Result<()> {
    println!("Compiling protos...");
    prost_build::compile_protos(&["protos/bed.proto"], &["src/", "protos/"])?;
    Ok(())
}