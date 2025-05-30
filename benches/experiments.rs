use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn access_0(
    storage: &mut [u64],
    offset: usize,
    index_size: usize,
    indices_per_u64: usize,
    index: usize,
) -> usize {
    let target_u64 = &mut storage[offset / indices_per_u64];
    let target_offset = 64 - (offset % indices_per_u64 + 1) * index_size;
    let mask = u64::MAX >> (64 - index_size);
    let old_index = (*target_u64 >> target_offset) & mask;
    *target_u64 &= !(mask << target_offset);
    *target_u64 |= (index as u64) << target_offset;
    old_index as usize
}

fn access_1(
    storage: &mut [u64],
    offset: usize,
    index_size: usize,
    indices_per_u64: usize,
    index: usize,
) -> usize {
    let target_u64 = unsafe { storage.get_unchecked_mut(offset / indices_per_u64) };
    let target_offset = 64 - (offset % indices_per_u64 + 1) * index_size;
    let mask = u64::MAX >> (64 - index_size);
    let old_index = (*target_u64 >> target_offset) & mask;
    *target_u64 &= !(mask << target_offset);
    *target_u64 |= (index as u64) << target_offset;
    old_index as usize
}

fn experiment(c: &mut Criterion) {
    let mut group = c.benchmark_group("access");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(1000);
    group.warm_up_time(Duration::from_millis(1_000));

    const STORAGE_LEN: usize = 1024;
    let index_size = 8;
    let indices_per_u64 = 64 / index_size;
    let new_index = 0xAB;

    // Test a variety of offsets, including start, end, and mid–buffer.
    let offsets = [
        0,
        1,
        indices_per_u64 - 1,
        indices_per_u64,
        STORAGE_LEN * indices_per_u64 / 2,
        STORAGE_LEN * indices_per_u64 - 1,
    ];

    let mut storage = vec![0u64; STORAGE_LEN];

    for &offset in &offsets {
        // bounds-checked version
        group.bench_with_input(BenchmarkId::new("access_0", offset), &offset, |b, &off| {
            b.iter(|| {
                black_box(access_0(
                    black_box(&mut storage),
                    black_box(off),
                    black_box(index_size),
                    black_box(indices_per_u64),
                    black_box(new_index),
                ))
            })
        });

        // unsafe-indexed version
        group.bench_with_input(BenchmarkId::new("access_1", offset), &offset, |b, &off| {
            b.iter(|| {
                black_box(access_1(
                    black_box(&mut storage),
                    black_box(off),
                    black_box(index_size),
                    black_box(indices_per_u64),
                    black_box(new_index),
                ))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, experiment);
criterion_main!(benches);
