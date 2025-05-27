use crate::palette::{Palette, PaletteEntry};

mod hybrid;

fn test_palette_insert_new<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        assert_eq!(
            palette.insert_new(PaletteEntry { value, count: 1 }),
            value as u64
        );
    }
}

fn test_palette_len<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry { value, count: 1 });
        assert_eq!(palette.len(), value as usize + 1);
    }
}

fn test_pallete_get_by_value<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for value in 0..amount_unique_inserts {
        assert_eq!(
            palette.get_by_value(&value),
            Some(&PaletteEntry { value, count: 1 })
        );
        assert_eq!(
            palette.get_mut_by_value(&value),
            Some(&mut PaletteEntry { value, count: 1 })
        );
    }
}

fn test_palette_get_by_index<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for index in 0..amount_unique_inserts as u64 {
        assert_eq!(
            palette.get_by_index(index),
            Some(&PaletteEntry {
                value: index as i32,
                count: 1
            })
        );
        assert_eq!(
            palette.get_mut_by_index(index),
            Some(&mut PaletteEntry {
                value: index as i32,
                count: 1
            })
        );
    }
}

fn test_palette_get_index_from_value<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for value in 0..amount_unique_inserts {
        assert_eq!(palette.get_index_from_value(&value), Some(value as u64));
    }
}

fn test_palette_mark_as_unused<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for index in 0..amount_unique_inserts as u64 {
        palette.get_mut_by_index(index).unwrap().count = 0;
        palette.mark_as_unused(index);
    }
}

fn test_palette_mark_as_unused_len<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    assert_eq!(palette.len(), amount_unique_inserts as usize);

    for index in 0..amount_unique_inserts as u64 {
        palette.get_mut_by_index(index).unwrap().count = 0;
        palette.mark_as_unused(index);
        assert_eq!(
            palette.len(),
            amount_unique_inserts as usize - index as usize - 1
        );
    }
    assert_eq!(palette.len(), 0);
}

fn test_palette_optimize<P: Palette<i32>>(mut palette: P, amount_unique_inserts: i32) {
    let mut control = Vec::new();
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value,
            count: value as u32 + 1,
        });
        control.push(PaletteEntry {
            value,
            count: value as u32 + 1,
        });
    }

    let old_len = palette.len();
    palette.optimize();
    assert_eq!(palette.len(), old_len);
    for i in (0..palette.len() as u64).step_by(2) {
        palette.get_mut_by_value(&(i as i32)).unwrap().count = 0;
        palette.mark_as_unused(i);
    }
    for i in (0..control.len()).rev().step_by(2) {
        control.remove(i);
    }

    palette.optimize();

    for control_value in control {
        assert_eq!(
            palette
                .get_mut_by_value(&control_value.value)
                .unwrap()
                .count,
            control_value.count
        );
    }
}
