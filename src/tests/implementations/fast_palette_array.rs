use crate::{fast_palette_array::FastPaletteArray, tests::cases::test_palette_array_get_set_rng};

#[test]
fn test_fast_palette_array_get_set_rng() {
    let array = FastPaletteArray::<i32>::new(512, 0);
    test_palette_array_get_set_rng(array);
}
