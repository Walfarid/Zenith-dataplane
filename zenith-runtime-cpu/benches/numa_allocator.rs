//! NUMA allocator benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zenith_runtime_cpu::allocator::{AllocatorConfig, NumaAllocator};
use std::alloc::Layout;

fn benchmark_allocate(c: &mut Criterion) {
    let allocator = NumaAllocator::with_defaults();
    let layout = Layout::from_size_align(1024, 8).unwrap();
    
    c.bench_function("numa_allocate_1kb", |b| {
        b.iter(|| unsafe {
            let ptr = allocator.allocate(black_box(layout)).unwrap();
            allocator.deallocate(ptr, layout);
        })
    });
}

criterion_group!(benches, benchmark_allocate);
criterion_main!(benches);
