use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
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
            black_box(vec.optimize());
        }
        if rng.random_bool(0.2) {
            let n = rng.random_range(0..unique_values);
            black_box(vec.push(n));
        }
        if rng.random_bool(0.2) {
            let n = rng.random_range(0..unique_values);
            black_box(vec.push_ref(&n));
        }
        if rng.random_bool(0.33) {
            black_box(vec.pop());
        }
        if rng.random_bool(0.5) && vec.len() > 0 {
            let index = rng.random_range(0..vec.len());
            let n = rng.random_range(0..unique_values);
            black_box(vec.set(index, &n));
        }
        if rng.random_bool(0.5) && vec.len() > 0 {
            let index = rng.random_range(0..vec.len());
            black_box(vec.get(index));
        }
    }
}

// Helper function to make defining benchmarks for different THRESHOLDs easier
#[allow(dead_code)]
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

#[allow(dead_code)]
fn rng_routine_benchmarks(c: &mut Criterion) {
    let iterations_per_call = 4000;

    benchmark_for_threshold::<1>(c, "rng_routine", iterations_per_call, &[1, 16]);
    benchmark_for_threshold::<4>(c, "rng_routine", iterations_per_call, &[1, 2, 3, 4, 5]);
    benchmark_for_threshold::<8>(c, "rng_routine", iterations_per_call, &[8, 9]);
    benchmark_for_threshold::<16>(c, "rng_routine", iterations_per_call, &[16, 17]);
    benchmark_for_threshold::<32>(c, "rng_routine", iterations_per_call, &[24, 32, 33]);
    benchmark_for_threshold::<64>(c, "rng_routine", iterations_per_call, &[40, 50, 64, 65]);
    benchmark_for_threshold::<256>(c, "rng_routine", iterations_per_call, &[128, 256, 512]);
}

// ------------------------------ GET OPERATION BENCHMARK --------------------------------------

// Helper function to define benchmarks for PaletteVec::get() for different THRESHOLDs
#[allow(dead_code)]
fn benchmark_get_for_threshold<const THRESHOLD: usize>(
    c: &mut Criterion,
    vec_len_for_setup: u32,
    get_iterations_per_call: u32,
    unique_values_options: &[u32],
) {
    let mut group = c.benchmark_group(format!("get_T{}", THRESHOLD));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(200));

    for &unique_vals in unique_values_options.iter() {
        if unique_vals == 0 {
            continue;
        }
        if vec_len_for_setup == 0 {
            continue;
        }

        group.bench_with_input(
            BenchmarkId::new("uv", unique_vals),
            &(unique_vals, vec_len_for_setup, get_iterations_per_call),
            |b, &(uv, v_len, get_iters)| {
                b.iter_batched_ref(
                    || {
                        let mut setup_rng = Xoshiro256PlusPlus::seed_from_u64(42983481910120131);
                        let mut pv: PaletteVec<
                            u32,
                            HybridPalette<THRESHOLD, u32>,
                            AlignedIndexBuffer,
                        > = PaletteVec::new();

                        for _ in 0..v_len {
                            let val = setup_rng.random_range(0..uv);
                            pv.push(val);
                        }
                        pv
                    },
                    |pv_ref| {
                        for i in 0..get_iters {
                            let index = i as usize % pv_ref.len();
                            black_box(pv_ref.get(index));
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

#[allow(dead_code)]
fn get_routine_benchmarks(c: &mut Criterion) {
    const VEC_LEN_FOR_SETUP: u32 = 4000;
    const GET_ITERATIONS_PER_CALL: u32 = 4000;

    // Test cases similar to rng_routine_benchmarks, focusing on unique values around THRESHOLD
    benchmark_get_for_threshold::<1>(c, VEC_LEN_FOR_SETUP, GET_ITERATIONS_PER_CALL, &[1, 2, 8]); // Test with more unique values than threshold
    benchmark_get_for_threshold::<4>(c, VEC_LEN_FOR_SETUP, GET_ITERATIONS_PER_CALL, &[1, 4, 5]);
    benchmark_get_for_threshold::<8>(
        c,
        VEC_LEN_FOR_SETUP,
        GET_ITERATIONS_PER_CALL,
        &[4, 8, 9, 16],
    );
    benchmark_get_for_threshold::<16>(c, VEC_LEN_FOR_SETUP, GET_ITERATIONS_PER_CALL, &[15, 16, 17]);
    benchmark_get_for_threshold::<32>(c, VEC_LEN_FOR_SETUP, GET_ITERATIONS_PER_CALL, &[32, 33, 64]);
    benchmark_get_for_threshold::<64>(
        c,
        VEC_LEN_FOR_SETUP,
        GET_ITERATIONS_PER_CALL,
        &[1, 8, 16, 63, 64, 65],
    );
    benchmark_get_for_threshold::<256>(
        c,
        VEC_LEN_FOR_SETUP,
        GET_ITERATIONS_PER_CALL,
        &[1, 8, 16, 104, 146, 255, 256, 257, 512],
    );
}

// ------------------------------ SET OPERATION BENCHMARK --------------------------------------

// Helper function to define benchmarks for PaletteVec::set() for different THRESHOLDs
fn benchmark_set_for_threshold<const THRESHOLD: usize>(
    c: &mut Criterion,
    vec_len_for_setup: u32,
    get_iterations_per_call: u32,
    unique_values_options: &[u32],
) {
    let mut group = c.benchmark_group(format!("set_T{}", THRESHOLD));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(30); // Increased sample size
    group.warm_up_time(Duration::from_millis(200));

    for &unique_vals in unique_values_options.iter() {
        if unique_vals == 0 {
            continue;
        }
        if vec_len_for_setup == 0 {
            continue;
        }

        group.bench_with_input(
            BenchmarkId::new("uv", unique_vals),
            &(unique_vals, vec_len_for_setup, get_iterations_per_call),
            |b, &(uv, v_len, get_iters)| {
                b.iter_batched_ref(
                    || {
                        let pv: PaletteVec<u32, HybridPalette<THRESHOLD, u32>, AlignedIndexBuffer> =
                            PaletteVec::filled(0, v_len as usize);
                        pv
                    },
                    |pv_ref| {
                        for i in 0..get_iters {
                            let index = i as usize % pv_ref.len();
                            let value = i as u32 % uv;
                            black_box(pv_ref.set(index, &value));
                        }
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

fn set_routine_benchmarks(c: &mut Criterion) {
    const VEC_LEN_FOR_SETUP: u32 = 4000;
    const SET_ITERATIONS_PER_CALL: u32 = 4000;

    // Test cases similar to rng_routine_benchmarks, focusing on unique values around THRESHOLD
    benchmark_set_for_threshold::<1>(c, VEC_LEN_FOR_SETUP, SET_ITERATIONS_PER_CALL, &[1, 2, 8]); // Test with more unique values than threshold
    benchmark_set_for_threshold::<4>(c, VEC_LEN_FOR_SETUP, SET_ITERATIONS_PER_CALL, &[1, 4, 5]);
    benchmark_set_for_threshold::<8>(
        c,
        VEC_LEN_FOR_SETUP,
        SET_ITERATIONS_PER_CALL,
        &[4, 8, 9, 16],
    );
    benchmark_set_for_threshold::<16>(
        c,
        VEC_LEN_FOR_SETUP,
        SET_ITERATIONS_PER_CALL,
        &[4, 8, 12, 16, 17],
    );
    benchmark_set_for_threshold::<32>(c, VEC_LEN_FOR_SETUP, SET_ITERATIONS_PER_CALL, &[32, 33, 64]);
    benchmark_set_for_threshold::<64>(
        c,
        VEC_LEN_FOR_SETUP,
        SET_ITERATIONS_PER_CALL,
        &[1, 8, 16, 63, 64, 65],
    );
    benchmark_set_for_threshold::<256>(
        c,
        VEC_LEN_FOR_SETUP,
        SET_ITERATIONS_PER_CALL,
        &[1, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 255, 256, 257, 512],
    );
}

//criterion_group!(benches, rng_routine_benchmarks);
//criterion_group!(benches, get_routine_benchmarks);
criterion_group!(benches, set_routine_benchmarks);
criterion_main!(benches);
