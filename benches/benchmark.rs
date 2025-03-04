use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use palettevec::PaletteVec;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const AMOUNT_DIFFERENT_ITEMS: u32 = 33;
const RNG_SEED: u64 = 44444444;
const NUM_PUSHES: u32 = 1_000_000;
const NUM_POPS: u32 = 1_000_000;
const NUM_SETS: u32 = 1_000_000;
const NUM_GETS: u32 = 1_000_000;
const NUM_RANDOM_ACTIONS: u32 = 48_000;
const SAMPLE_SIZE: usize = 300;

#[inline]
fn create_empty_palette_vec() -> PaletteVec<u32> {
    PaletteVec::new()
}

fn push_random_u8s(n: u32, rng: &mut impl Rng) -> PaletteVec<u8> {
    let mut vec = PaletteVec::new();
    for _ in 0..n {
        // Generate a random u8 by taking modulo 256 of a random u32.
        vec.push((rng.next_u32() % AMOUNT_DIFFERENT_ITEMS) as u8);
    }
    vec
}

fn pop_u8s(n: u32, vec: &mut PaletteVec<u8>) {
    for _ in 0..n {
        vec.pop();
    }
}

fn set_random(n: u32, vec: &mut PaletteVec<u8>, rng: &mut impl Rng) {
    for _ in 0..n {
        let idx = rng.random_range(0..vec.len());
        vec.set(idx, (rng.next_u32() % AMOUNT_DIFFERENT_ITEMS) as u8);
    }
}

fn get_random(n: u32, vec: &PaletteVec<u8>, rng: &mut impl Rng) {
    for _ in 0..n {
        let idx = rng.random_range(0..vec.len());
        black_box(vec.get(idx));
    }
}

fn random_actions(n: u32, rng: &mut impl Rng) -> PaletteVec<u8> {
    let mut vec = PaletteVec::new();
    for _ in 0..n {
        if rng.random_bool(0.5) {
            vec.push((rng.next_u32() % AMOUNT_DIFFERENT_ITEMS) as u8);
        }

        if !vec.is_empty() && rng.random_bool(0.2) {
            vec.pop();
        }

        if !vec.is_empty() && rng.random_bool(0.5) {
            let idx = rng.random_range(0..vec.len());
            vec.set(idx, (rng.next_u32() % AMOUNT_DIFFERENT_ITEMS) as u8);
        }

        if !vec.is_empty() && rng.random_bool(0.5) {
            let idx = rng.random_range(0..vec.len());
            black_box(vec.get(idx));
        }

        if !vec.is_empty() && rng.random_bool(0.012) {
            vec.optimize();
        }

        if !vec.is_empty() && rng.random_bool(0.1) {
            vec.remove(rng.random_range(0..vec.len()));
        }

        if !vec.is_empty() && rng.random_bool(0.1) {
            vec.insert(rng.random_range(0..vec.len()), (rng.next_u32() % AMOUNT_DIFFERENT_ITEMS) as u8);
        }
    }
    vec
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("PaletteVec<u8>");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("Empty PaletteVec<u32> creation", |b| {
        b.iter(|| black_box(create_empty_palette_vec()))
    });

    group.bench_function(&format!("Pushing {} random u8s", NUM_PUSHES), |b| {
        b.iter_batched(
            || ChaCha8Rng::seed_from_u64(RNG_SEED),
            |mut rng| black_box(push_random_u8s(NUM_PUSHES, &mut rng)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(&format!("Popping {} random u8s", NUM_PUSHES), |b| {
        b.iter_batched(
            || {
                let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
                push_random_u8s(NUM_POPS, &mut rng)
            },
            |mut vec| pop_u8s(NUM_POPS, &mut vec),
            BatchSize::LargeInput,
        )
    });

    group.bench_function(&format!("Setting {} random u8s", NUM_SETS), |b| {
        b.iter_batched(
            || {
                let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
                push_random_u8s(50000, &mut rng)
            },
            |mut vec| {
                let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
                set_random(NUM_SETS, &mut vec, &mut rng);
                black_box(vec)
            },
            BatchSize::LargeInput,
        )
    });

    group.bench_function(&format!("Getting {} random u8s", NUM_GETS), |b| {
        b.iter_batched(
            || {
                let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
                push_random_u8s(50000, &mut rng)
            },
            |vec| {
                let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
                get_random(NUM_GETS, &vec, &mut rng);
            },
            BatchSize::LargeInput,
        )
    });

    group.bench_function(&format!("Random actions with {} actions", NUM_RANDOM_ACTIONS), |b| {
        b.iter_batched(
            || ChaCha8Rng::seed_from_u64(RNG_SEED),
            |mut rng| black_box(random_actions(NUM_RANDOM_ACTIONS, &mut rng)),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
