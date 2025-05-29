mod index_buffer;
mod palette;
mod palette_vec;

/// Controls the amount of iterations that RNG tests do.
/// 0.0 means 0 iterations, 1.0 means base line,
/// 2.0 means twice the base line etc.
const RNG_TEST_EXTENSIVENESS: f64 = 1.0;

fn calc_rng_iterations(base: usize) -> usize {
    (base as f64 * RNG_TEST_EXTENSIVENESS) as usize
}
