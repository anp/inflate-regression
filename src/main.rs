#[macro_use]
extern crate criterion;
extern crate inflate;

use criterion::{Criterion, black_box};
use inflate::inflate_bytes;

fn decode(c: &mut Criterion) {
    c.bench_function("decode", |b| {
        let compressed = include_bytes!("./1m_random_deflated");
        b.iter(|| black_box(inflate_bytes(compressed).unwrap()));
    });
}

criterion_group!(inflate, decode);
criterion_main!(inflate);
