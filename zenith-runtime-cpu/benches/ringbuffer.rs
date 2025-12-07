//! Ring buffer benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zenith_runtime_cpu::buffer::{RingBuffer, SpscRingBuffer};

fn benchmark_spsc_push(c: &mut Criterion) {
    let buffer = SpscRingBuffer::<u64>::new(65536);
    
    c.bench_function("spsc_push", |b| {
        b.iter(|| {
            let _ = buffer.try_push(black_box(42));
            let _ = buffer.try_pop();
        })
    });
}

fn benchmark_spsc_throughput(c: &mut Criterion) {
    let buffer = SpscRingBuffer::<u64>::new(65536);
    
    c.bench_function("spsc_throughput_1000", |b| {
        b.iter(|| {
            for i in 0..1000 {
                let _ = buffer.try_push(black_box(i));
            }
            for _ in 0..1000 {
                let _ = buffer.try_pop();
            }
        })
    });
}

criterion_group!(benches, benchmark_spsc_push, benchmark_spsc_throughput);
criterion_main!(benches);
