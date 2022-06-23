use noodles_bed as bed;
use std::{env, error::Error, fs, path::Path, sync::Arc, io::{Read, Write}};


// // This is what it seems like we should do
// struct ModernBioFormat<F1: BioSerdeRead, F2: BioSerdeWrite> {
//     reader: F1,
//     writer: F2,
//     origin: String,
//     destination: String,
// }

// // How to represent "Bed" here?
// //      we may even have an enum of ImplementedFormats
// impl BioSerdeRead for Bed {...}
// impl BioSerdeWrite for Bed {...}

// This experiment doesn't work:
//      we would need a generic trait that says
//      that says that there Writers and Readers are already taken care of
//      by our code
struct ModernBioFormat<F1: Read, F2: Write> {
    reader: F1,
    writer: F2,
    origin: String,
    destination: String,
}

pub fn run<F1: Read, F2: Write>(mbf: ModernBioFormat<F1, F2>) -> Result<(), Box<dyn Error>> {
    let bed_records_vec = mbf
        .reader
        // This is a specific Bed Reading mechanism
        //      that is bleeding in a generic part of the code
        .records::<3>() 
        .filter_map(|record| record.ok())
        .collect();

    for record in bed_records_vec.iter() {
        // This points out a problem:
        //      even if we solve generic writers and readers
        //      the bulk of our work will be to make the <write_record> itself
        //
        // In the specific case of Bed, it asks for specifically a bed::Record type.
        //      which is a problem, because it forces our hand to conform
        //      to a specific implementation
        //      whereas being able to receive a generic trait of some sorts
        //      would enable us to pass the IR main struct directly
        //      (maybe we can already rehearse making a noodles PR in that sense
        //          but that would require us to already have the IR definition)
        mbf.writer.write_record::<3>(&record)?;    
    }

    Ok(())
}

fn main() {
    let input_file = "samples/well_formed_sample.bed".to_string();
    let output_file = "output.bed".to_string();

    let data = fs::read(&mbf.origin)?;
    let mut reader = bed::Reader::new(&data[..]);

    let path = Path::new(&destination);
    let file = fs::File::create(&path)?;
    let mut writer = bed::Writer::new(file);

    let mbf = ModernBioFormat {
        reader,
        writer,
        origin: input_file,
        destination: output_file,
    };

    // identity test. read and write the same bed.
    run(mbf);

}