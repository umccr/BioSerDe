# BioSerDe

This is a PoC for the [GSoC 2022 idea from GA4GH][bioserde_twitter].

At present this repo just codegens [`.proto` files borrowed from Google nucleus][google_nucleus], but the intention is
to [make it SerDe friendly][prost_serde] and eventually become a hub or "Rosetta stone" of [bioinformatics format cross-conversion by leveraging Noodles as well][noodles_serde].

## Usage

```
% cargo run
   Compiling bioserde v0.1.0 (/Users/rvalls/dev/umccr/bioserde)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/bioserde`
[src/main.rs:6] create_simple_bedfile("chr1") = BedRecord {
    reference_name: "",
    start: 0,
    end: 0,
    name: "",
    score: 0.0,
    strand: NoStrand,
    thick_start: 0,
    thick_end: 0,
    item_rgb: "",
    block_count: 0,
    block_sizes: "",
    block_starts: "",
}
```

[prost_serde]: https://github.com/tokio-rs/prost#faq
[noodles_serde]: https://github.com/zaeleus/noodles/issues/53
[bioserde_twitter]: https://twitter.com/UMCCR/status/1511598211034624000
[google_nucleus]: https://github.com/google/nucleus/tree/v0.6.0/nucleus/protos
