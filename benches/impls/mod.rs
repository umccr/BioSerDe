use std::{fs::File, io::BufReader, io::sink};
use noodles::bed;

use crate::bioformat::BioFormat;


//TODO: Dynamic Sizing over Record<Size>
//TODO: Accept Arbitrary Records

impl BioFormat for bed::record::Record<3> {
    fn read_whole(&self) -> Result<(), Box<dyn std::error::Error>> {

        let mut reader = File::open("testdata/multi-reference.bam").map(BufReader::new).map(bed::Reader::new)?;

        for r in &self {
            let rec = r?;
            sink.send(rec);
        }

        Ok(())
    }

    fn read_header(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {

        //Construct a writer to a sink, so not clutter any actual output
        let mut writer = bed::Writer::new(io::sink());

        writer.write_record(&self).map_err(|e| e.into())

    }
}

/*impl<const N: u8> BioFormat for bed::record::Record<N> {
    fn read_whole(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {

        //Construct a writer to a sink, so not clutter any actual output
        let mut writer = bed::Writer::new(io::sink());

        writer.write_record(&self).map_err(|e| e.into())

    }
}*/