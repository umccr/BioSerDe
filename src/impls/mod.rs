use std::{fs::File, io::BufReader, io::sink};
use std::io::Write;
use noodles::bed;
use noodles::fastq;

use crate::bioformat::BioFormat;
use crate::bioformat::FileError;


//TODO: Dynamic Sizing over Record<Size>

//impl<const N: u8> BioFormat for bed::record::Record<N>
impl BioFormat for bed::record::Record<3> {

    fn load_whole(&self, path: &std::path::Path) -> Result<(), FileError> {

        let mut reader = File::open(path).map(BufReader::new).map(bed::Reader::new).map_err(FileError::FileOpen)?;

        for result in reader.records::<3>() {
            let rec = result.map_err(FileError::RecordRead)?;

            //write to a sink, to avoid the reads from being optimized away
            //Criterion blackbox might take care of this
            sink().write(rec.to_string().as_bytes()).map_err(FileError::FileSave)?;
        }

        Ok(())
    }

    fn load_header(&self, path: &std::path::Path) -> Result<(), FileError> {
        todo!()
    }

    fn save(&self, path: &std::path::Path) -> Result<(), FileError> {

        //Construct a writer to a sink, to not clutter any actual output/stdout
        let mut writer = bed::Writer::new(sink());
        writer.write_record(&self).map_err(FileError::RecordRead)?;

        Ok(())

    }
}

impl BioFormat for fastq::Record {
    //TODO: Think over this. If we're reading in new records, are we even leveraging the self record?
    fn load_whole(&self, path: &std::path::Path) -> Result<(), FileError> {
        let reader = File::open(path).map(BufReader::new).map(bed::Reader::new).map_err(FileError::FileOpen)?;

        Ok(())
    }

    fn load_header(&self, path: &std::path::Path) -> Result<(), FileError> {
        todo!()
    }

    fn save(&self, path: &std::path::Path) -> Result<(), FileError> {
        let mut writer = fastq::Writer::new(sink());
        writer.write_record(&self).map_err(FileError::RecordWrite)?;

        Ok(())
    }
}