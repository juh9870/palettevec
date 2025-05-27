use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::PaletteArray;

use super::TEST_EXTENSIVENESS;

fn calc_iterations(base: usize) -> usize {
    (base as f64 * TEST_EXTENSIVENESS) as usize
}

pub fn test_palette_array_get_set_rng<P: PaletteArray<i32>>(mut array: P) {
    let mut rng = ChaCha8Rng::seed_from_u64(832723423458321);
    let mut control = Vec::new();

    if array.len() == 0 {
        return;
    }

    // Make both equal
    for i in 0..array.len() {
        array.set(i, &0);
        control.push(0);
    }

    // Test random operations
    for _ in 0..calc_iterations(64000) {
        // Get random and check equality
        let index = rng.random_range(0..array.len());
        assert_eq!(*array.get(index), control[index]);

        // Set random
        let index = rng.random_range(0..array.len());
        let n = rng.random_range(0..257);
        array.set(index, &n);
        control[index] = n;
    }

    // Check equality
    for i in 0..array.len() {
        assert_eq!(*array.get(i), control[i]);
    }
}
