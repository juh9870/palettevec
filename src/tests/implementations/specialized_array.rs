use crate::{tests::cases::test_palette_array_get_set_rng, SpecializedPaletteArray};

#[test]
fn test_specialized_array_get_set_rng() {
    let array = SpecializedPaletteArray::<i32>::new(512, 0);
    test_palette_array_get_set_rng(array);
}
