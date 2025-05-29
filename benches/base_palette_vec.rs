use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use palettevec::{
    index_buffer::aligned::AlignedIndexBuffer, palette::hybrid::HybridPalette, PaletteVec,
};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

// ------------------------------ RNG OPERATION BENCHMARK --------------------------------------

pub fn rng_routine<const THRESHOLD: usize>(unique_values: u32, iterations: u32) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42983481910120131);
    let mut vec: PaletteVec<u32, HybridPalette<THRESHOLD, u32>, AlignedIndexBuffer> =
        PaletteVec::new();

    for _ in 0..iterations {
        if rng.random_bool(0.008) {
            vec.optimize();
        }
        if rng.random_bool(0.2) {
            let n = rng.random_range(0..unique_values);
            vec.push(n);
        }
        if rng.random_bool(0.2) {
            let n = rng.random_range(0..unique_values);
            vec.push_ref(&n);
        }
        if rng.random_bool(0.33) {
            vec.pop();
        }
        if rng.random_bool(0.5) && vec.len() > 0 {
            let index = rng.random_range(0..vec.len());
            let n = rng.random_range(0..unique_values);
            vec.set(index, &n);
        }
        if rng.random_bool(0.5) && vec.len() > 0 {
            let index = rng.random_range(0..vec.len());
            vec.get(index);
        }
    }
}

// Helper function to make defining benchmarks for different THRESHOLDs easier
fn benchmark_for_threshold<const THRESHOLD: usize>(
    c: &mut Criterion,
    group_name_prefix: &str,
    iterations_per_call: u32,
    unique_values_options: &[u32],
) {
    let mut group = c.benchmark_group(format!("{}_T{}", group_name_prefix, THRESHOLD));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(200));

    for &unique_vals in unique_values_options.iter() {
        if unique_vals == 0 {
            continue;
        }

        group.bench_with_input(
            BenchmarkId::new("uv", unique_vals),
            &unique_vals,
            |b, &uv| {
                b.iter(|| {
                    // black_box is used to prevent the compiler from optimizing
                    // away the routine or its parameters.
                    rng_routine::<THRESHOLD>(black_box(uv), black_box(iterations_per_call));
                });
            },
        );
    }
    group.finish();
}

fn rng_routine_benchmarks(c: &mut Criterion) {
    let iterations_per_call = 4000;

    benchmark_for_threshold::<1>(c, "rng_routine", iterations_per_call, &[1, 16]);
    benchmark_for_threshold::<4>(c, "rng_routine", iterations_per_call, &[1, 2, 3, 4]);
    benchmark_for_threshold::<8>(c, "rng_routine", iterations_per_call, &[8, 9]);
    benchmark_for_threshold::<16>(c, "rng_routine", iterations_per_call, &[16, 17]);
    benchmark_for_threshold::<32>(c, "rng_routine", iterations_per_call, &[24, 32, 33]);
    benchmark_for_threshold::<64>(c, "rng_routine", iterations_per_call, &[40, 50, 64, 65]);
    benchmark_for_threshold::<256>(c, "rng_routine", iterations_per_call, &[128, 256, 512]);
}

criterion_group!(benches, rng_routine_benchmarks);
criterion_main!(benches);
