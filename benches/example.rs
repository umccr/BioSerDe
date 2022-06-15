use criterion::{black_box, Criterion, criterion_group, criterion_main};
use noodles::bed;
use noodles::core::Position;

use bioserde::bioformat::BioSerde;


fn bioformat_write(form: impl BioSerde) -> Result<(), Box<dyn std::error::Error>> {
    form.write()
}

fn bioformat_read_whole(form: impl BioSerde, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    form.read_whole(filename)
}

fn criterion_benchmark(c: &mut Criterion) {
    //We demonstrate a bed::Record with BioFormat
    //Example record from noodles-bed

    let record = bed::Record::<3>::builder()
        .set_reference_sequence_name("sq0")
        .set_start_position(Position::try_from(8).unwrap())
        .set_end_position(Position::try_from(13).unwrap())
        .build().unwrap();

    //TODO: Utilize benchmark_with_input

    c.bench_function("Bam Write Example", |b| b.iter(|| bioformat_write(black_box(record.clone()))));

    c.bench_function("Bam Read Example", |b| b.iter(|| bioformat_read_whole(black_box(record.clone()), "testdata/multi-reference.bam")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);