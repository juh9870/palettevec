use crate::{index_buffer::aligned::AlignedIndexBuffer, palette::hybrid::HybridPalette};

use super::*;

#[test]
fn base_palette_vec_new() {
    test_palette_vec_new::<HybridPalette<0, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<1, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<2, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<3, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<4, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<17, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<49, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<199, i32>, AlignedIndexBuffer>();
    test_palette_vec_new::<HybridPalette<2000, i32>, AlignedIndexBuffer>();
}

#[test]
fn base_palette_vec_push_pop() {
    test_palette_vec_push_pop::<HybridPalette<0, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<1, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<2, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<3, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<4, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<17, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<49, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<199, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<2000, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_pop::<HybridPalette<16, u32>, AlignedIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_push_ref_pop() {
    test_palette_vec_push_ref_pop::<HybridPalette<0, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<1, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<2, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<3, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<4, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<17, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<49, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<199, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<2000, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_push_ref_pop::<HybridPalette<16, u32>, AlignedIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_len() {
    test_palette_vec_len::<HybridPalette<0, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<1, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<2, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<3, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<4, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<17, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<49, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<199, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<2000, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_len::<HybridPalette<16, u32>, AlignedIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_unique_values() {
    test_palette_vec_unique_values::<HybridPalette<0, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<1, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<2, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<3, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<4, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<17, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<49, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<199, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<2000, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_unique_values::<HybridPalette<16, u32>, AlignedIndexBuffer>(445, 3333);
}

#[test]
fn base_palette_vec_set() {
    test_palette_vec_set::<HybridPalette<0, u32>, AlignedIndexBuffer>(32, 3333);
    test_palette_vec_set::<HybridPalette<1, u32>, AlignedIndexBuffer>(444, 3333);
    test_palette_vec_set::<HybridPalette<2, u32>, AlignedIndexBuffer>(23, 3333);
    test_palette_vec_set::<HybridPalette<3, u32>, AlignedIndexBuffer>(5, 3333);
    test_palette_vec_set::<HybridPalette<4, u32>, AlignedIndexBuffer>(76, 3333);
    test_palette_vec_set::<HybridPalette<17, u32>, AlignedIndexBuffer>(7, 3333);
    test_palette_vec_set::<HybridPalette<49, u32>, AlignedIndexBuffer>(112, 3333);
    test_palette_vec_set::<HybridPalette<199, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_set::<HybridPalette<2000, u32>, AlignedIndexBuffer>(444, 3333);
    test_palette_vec_set::<HybridPalette<16, u32>, AlignedIndexBuffer>(31, 3333);
}

#[test]
fn base_palette_vec_get() {
    test_palette_vec_get::<HybridPalette<0, u32>, AlignedIndexBuffer>(32, 3333);
    test_palette_vec_get::<HybridPalette<1, u32>, AlignedIndexBuffer>(444, 3333);
    test_palette_vec_get::<HybridPalette<2, u32>, AlignedIndexBuffer>(23, 3333);
    test_palette_vec_get::<HybridPalette<3, u32>, AlignedIndexBuffer>(5, 3333);
    test_palette_vec_get::<HybridPalette<4, u32>, AlignedIndexBuffer>(76, 3333);
    test_palette_vec_get::<HybridPalette<17, u32>, AlignedIndexBuffer>(7, 3333);
    test_palette_vec_get::<HybridPalette<49, u32>, AlignedIndexBuffer>(112, 3333);
    test_palette_vec_get::<HybridPalette<199, u32>, AlignedIndexBuffer>(445, 3333);
    test_palette_vec_get::<HybridPalette<2000, u32>, AlignedIndexBuffer>(444, 3333);
    test_palette_vec_get::<HybridPalette<16, u32>, AlignedIndexBuffer>(31, 3333);
}

#[test]
fn base_palette_vec_filled() {
    test_palette_vec_filled::<HybridPalette<0, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<1, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<2, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<3, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<4, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<17, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<49, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<199, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<2000, u32>, AlignedIndexBuffer>(3333);
    test_palette_vec_filled::<HybridPalette<16, u32>, AlignedIndexBuffer>(3333);
}

#[test]
fn base_palette_vec_optimize() {
    test_palette_vec_optimize::<HybridPalette<0, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<1, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<2, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<3, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<4, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<17, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<49, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<199, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<2000, u32>, AlignedIndexBuffer>(7333);
    test_palette_vec_optimize::<HybridPalette<16, u32>, AlignedIndexBuffer>(7333);
}
