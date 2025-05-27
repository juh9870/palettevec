use crate::palette::hybrid::HybridPalette;

use super::{
    test_palette_get_by_index, test_palette_get_index_from_value, test_palette_index_size,
    test_palette_index_size_after_optimizing, test_palette_insert_new, test_palette_len,
    test_palette_mark_as_unused, test_palette_mark_as_unused_len, test_palette_optimize,
    test_pallete_get_by_value,
};

#[test]
fn palette_insert_new() {
    test_palette_insert_new(HybridPalette::<0, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<1, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<2, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<3, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<4, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<5, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<17, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<32, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<127, i32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_len() {
    test_palette_len(HybridPalette::<0, i32>::new(), 2049);
    test_palette_len(HybridPalette::<1, i32>::new(), 2049);
    test_palette_len(HybridPalette::<2, i32>::new(), 2049);
    test_palette_len(HybridPalette::<3, i32>::new(), 2049);
    test_palette_len(HybridPalette::<4, i32>::new(), 2049);
    test_palette_len(HybridPalette::<5, i32>::new(), 2049);
    test_palette_len(HybridPalette::<17, i32>::new(), 2049);
    test_palette_len(HybridPalette::<32, i32>::new(), 2049);
    test_palette_len(HybridPalette::<127, i32>::new(), 2049);
    test_palette_len(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_index_size() {
    test_palette_index_size(HybridPalette::<0, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<1, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<2, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<3, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<4, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<5, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<17, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<32, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<127, i32>::new(), 2049);
    test_palette_index_size(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_get_by_value() {
    test_pallete_get_by_value(HybridPalette::<0, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<1, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<2, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<3, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<4, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<5, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<17, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<32, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<127, i32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_get_by_index() {
    test_palette_get_by_index(HybridPalette::<0, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<1, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<2, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<3, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<4, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<5, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<17, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<32, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<127, i32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_get_index_from_value() {
    test_palette_get_index_from_value(HybridPalette::<0, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<1, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<2, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<3, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<4, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<5, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<17, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<32, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<127, i32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_mark_as_unused() {
    test_palette_mark_as_unused(HybridPalette::<0, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<1, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<2, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<3, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<4, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<5, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<17, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<32, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<127, i32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_mark_as_unused_len() {
    test_palette_mark_as_unused_len(HybridPalette::<0, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<1, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<2, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<3, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<4, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<5, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<17, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<32, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<127, i32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_optimize() {
    test_palette_optimize(HybridPalette::<0, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<1, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<2, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<3, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<4, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<5, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<17, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<32, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<127, i32>::new(), 2049);
    test_palette_optimize(HybridPalette::<33333, i32>::new(), 3589);
}

#[test]
fn palette_index_size_after_optimizing() {
    test_palette_index_size_after_optimizing(HybridPalette::<0, i32>::new(), 16);
    test_palette_index_size_after_optimizing(HybridPalette::<1, i32>::new(), 20);
    test_palette_index_size_after_optimizing(HybridPalette::<2, i32>::new(), 9);
    test_palette_index_size_after_optimizing(HybridPalette::<3, i32>::new(), 7);
    test_palette_index_size_after_optimizing(HybridPalette::<4, i32>::new(), 27);
    test_palette_index_size_after_optimizing(HybridPalette::<5, i32>::new(), 29);
    test_palette_index_size_after_optimizing(HybridPalette::<17, i32>::new(), 18);
    test_palette_index_size_after_optimizing(HybridPalette::<32, i32>::new(), 2);
    test_palette_index_size_after_optimizing(HybridPalette::<127, i32>::new(), 1);
    test_palette_index_size_after_optimizing(HybridPalette::<33333, i32>::new(), 249);
}
