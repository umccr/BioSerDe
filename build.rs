use prost_build;
use std::io::Result;

fn main() -> Result<()> {
    println!("Compiling protos...");
    prost_build::compile_protos(
        &["protos/bed.proto", "protos/fastq.proto"],
        &["src/", "protos/"],
    )?;
    Ok(())
}
