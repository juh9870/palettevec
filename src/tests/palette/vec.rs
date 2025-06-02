use crate::palette::{vec::VecPalette, Palette};

use super::*;

#[test]
fn palette_insert_new() {
    test_palette_insert_new(VecPalette::new(), 2049);
}

#[test]
fn palette_len() {
    test_palette_len(VecPalette::new(), 2049);
}

#[test]
fn palette_index_size() {
    test_palette_index_size(VecPalette::new(), 2049);
}

#[test]
fn palette_get_by_value() {
    test_pallete_get_by_value(VecPalette::new(), 2049);
}

#[test]
fn palette_get_by_index() {
    test_palette_get_by_index(VecPalette::new(), 2049);
}

#[test]
fn palette_mark_as_unused() {
    test_palette_mark_as_unused(VecPalette::new(), 2049);
}

#[test]
fn palette_mark_as_unused_len() {
    test_palette_mark_as_unused_len(VecPalette::new(), 2049);
}

#[test]
fn palette_optimize() {
    test_palette_optimize(VecPalette::new(), 2049);
}

#[test]
fn palette_index_size_after_optimizing() {
    test_palette_index_size_after_optimizing(VecPalette::new(), 16);
}

#[test]
fn palette_iter() {
    test_palette_iter(VecPalette::new(), 16);
}

#[test]
fn palette_iter_mut() {
    test_palette_iter_mut(VecPalette::new(), 16);
}
