use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use cuter_nucleotides::*;

criterion_group!(benches, bench_encode);
criterion_main!(benches);

fn bench_encode(c: &mut Criterion) {
    let src = fs::read("nucleotides.txt").unwrap();
    let mut dst = Vec::with_capacity(src.len() / 4 + 1);

    let mut group = c.benchmark_group("bench_encode");
    group.throughput(Throughput::Bytes(src.len() as u64));

    group.bench_function("mul_compress", |b| {
        b.iter(|| unsafe { encode_mul_compress(&src, dst.as_mut_ptr()) })
    });
    group.bench_function("bitshuffle", |b| {
        b.iter(|| unsafe { encode_bitshuffle(&src, dst.as_mut_ptr()) })
    });
    group.bench_function("movepi8_mask", |b| {
        b.iter(|| unsafe { encode_movepi8_mask(&src, dst.as_mut_ptr()) })
    });
    group.bench_function("avx2_movemask", |b| {
        b.iter(|| unsafe { encode_avx2_movemask(&src, dst.as_mut_ptr()) })
    });
    group.bench_function("bmi2_pext", |b| {
        b.iter(|| unsafe { encode_bmi2_pext(&src, dst.as_mut_ptr()) })
    });
}
