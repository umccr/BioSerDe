use std::{fs::File, io::BufReader, io::sink};
use std::error::Error;
use std::io::Write;
use noodles::bed;
use noodles::fastq;

use crate::bioformat::BioFormat;


//TODO: Dynamic Sizing over Record<Size>

//impl<const N: u8> BioFormat for bed::record::Record<N>
impl BioFormat for bed::record::Record<3> {
    fn read_whole(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {

        let mut reader = File::open(filename).map(BufReader::new).map(bed::Reader::new)?;

        for result in reader.records::<3>() {
            let rec = result?;

            //write to a sink, to avoid the reads from being optimized away
            //Criterion blackbox might take care of this

            sink().write(rec.to_string().as_bytes())?;
        }

        Ok(())
    }

    fn read_header(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {

        //Construct a writer to a sink, to not clutter any actual output/stdout
        let mut writer = bed::Writer::new(sink());
        writer.write_record(&self).map_err(|e| e.into())

    }
}

impl BioFormat for fastq::Record {
    //TODO: Think over this. If we're reading in new records, are we even leveraging the self record?
    fn read_whole(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = File::open(filename).map(BufReader::new).map(bed::Reader::new)?;

        Ok(())
    }

    fn read_header(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        let mut writer = fastq::Writer::new(sink());
        writer.write_record(&self).map_err(|e| e.into())
    }
}