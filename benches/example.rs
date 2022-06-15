use std::path::Path;
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use noodles::bed;
use noodles::core::Position;

use bioserde::bioformat::{BioFormat, FileError};


fn bioformat_save(form: &impl BioFormat, path: &std::path::Path) -> Result<(), FileError> {
    form.save(path)
}

fn bioformat_load_whole(form: &impl BioFormat, path: &std::path::Path) -> Result<(), FileError> {
    form.load_whole(path)
}

fn criterion_benchmark(c: &mut Criterion) {
    //We demonstrate a bed::Record with BioFormat
    //Example record from noodles-bed

    let record = bed::Record::<3>::builder()
        .set_reference_sequence_name("sq0")
        .set_start_position(Position::try_from(8).unwrap())
        .set_end_position(Position::try_from(13).unwrap())
        .build().unwrap();

    let example_data = Path::new("testdata/multi-reference.bam");

    //TODO: Utilize benchmark_with_input

    c.bench_function("Bam Write Example", |b| b.iter(|| bioformat_save(black_box(&record), example_data)));

    c.bench_function("Bam Read Example", |b| b.iter(|| bioformat_load_whole(black_box(&record), example_data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);