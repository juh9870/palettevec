use crate::{index_buffer::aligned::AlignedIndexBuffer, palette::hybrid::HybridPalette};

use super::{test_palette_vec_new, test_palette_vec_push_pop, test_palette_vec_push_ref_pop};

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
    test_palette_vec_push_pop::<HybridPalette<0, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<1, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<2, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<3, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<4, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<17, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<49, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<199, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<2000, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_pop::<HybridPalette<16, i32>, AlignedIndexBuffer>();
}

#[test]
fn base_palette_vec_push_ref_pop() {
    test_palette_vec_push_ref_pop::<HybridPalette<0, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<1, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<2, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<3, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<4, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<17, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<49, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<199, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<2000, i32>, AlignedIndexBuffer>();
    test_palette_vec_push_ref_pop::<HybridPalette<16, i32>, AlignedIndexBuffer>();
}
