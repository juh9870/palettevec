use crate::{index_buffer::fast::FastIndexBuffer, palette::vec::VecPalette};

use super::*;

#[test]
fn base_palette_vec_new() {
    test_palette_vec_new::<VecPalette<i32>, FastIndexBuffer>();
}

#[test]
fn base_palette_vec_push_pop() {
    test_palette_vec_push_pop::<VecPalette<u32>, FastIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_push_ref_pop() {
    test_palette_vec_push_ref_pop::<VecPalette<u32>, FastIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_len() {
    test_palette_vec_len::<VecPalette<u32>, FastIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_unique_values() {
    test_palette_vec_unique_values::<VecPalette<u32>, FastIndexBuffer>(445, 3333);
}

#[test]
fn base_palette_vec_set() {
    test_palette_vec_set::<VecPalette<u32>, FastIndexBuffer>(32, 3333);
}

#[test]
fn base_palette_vec_get() {
    test_palette_vec_get::<VecPalette<u32>, FastIndexBuffer>(32, 3333);
}

#[test]
fn base_palette_vec_filled() {
    test_palette_vec_filled::<VecPalette<u32>, FastIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_optimize() {
    test_palette_vec_optimize::<VecPalette<u32>, FastIndexBuffer>(7333);
}

#[test]
fn palette_vec_rng_operations() {
    let mut rng = ChaCha8Rng::seed_from_u64(492384923941);
    for _ in 0..calc_rng_iterations(32) {
        let seed = rng.random();
        test_palette_vec_rng_operations::<VecPalette<u32>, FastIndexBuffer>(seed, 7333);
    }
}

#[test]
fn palette_vec_iter() {
    test_palette_vec_iter::<VecPalette<u32>, FastIndexBuffer>(1, 1337);
}

#[test]
fn palette_vec_palette_iter() {
    test_palette_vec_palette_iter::<VecPalette<u32>, FastIndexBuffer>(1, 1337);
}

#[test]
fn palette_vec_palette_iter_mut() {
    test_palette_vec_palette_iter_mut::<VecPalette<u32>, FastIndexBuffer>(1, 1337);
}
