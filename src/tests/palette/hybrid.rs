use crate::palette::hybrid::HybridPalette;

use super::{
    test_palette_get_by_index, test_palette_get_index_from_value, test_palette_insert_new,
    test_palette_len, test_palette_mark_as_unused, test_palette_mark_as_unused_len,
    test_palette_optimize, test_pallete_get_by_value,
};

#[test]
fn palette_insert_new() {
    test_palette_insert_new(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_insert_new(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_len() {
    test_palette_len(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_len(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_get_by_value() {
    test_pallete_get_by_value(HybridPalette::<i32, 0>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 1>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 2>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 3>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 4>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 5>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 17>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 127>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_get_by_index() {
    test_palette_get_by_index(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_get_index_from_value() {
    test_palette_get_index_from_value(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_get_index_from_value(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_mark_as_unused() {
    test_palette_mark_as_unused(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_mark_as_unused_len() {
    test_palette_mark_as_unused_len(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<i32, 33333>::new(), 3589);
}

#[test]
fn palette_optimize() {
    test_palette_optimize(HybridPalette::<i32, 0>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 1>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 2>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 3>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 4>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 5>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 17>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 32>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 127>::new(), 2049);
    test_palette_optimize(HybridPalette::<i32, 33333>::new(), 3589);
}
