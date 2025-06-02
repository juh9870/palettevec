use rustc_hash::FxHashMap;

use crate::palette::{calculate_smallest_index_size, Palette, PaletteEntry};

mod hybrid;

fn test_palette_insert_new<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    for value in 0..amount_unique_inserts {
        assert_eq!(
            palette
                .insert_new(PaletteEntry {
                    value: value as u32,
                    count: 1
                })
                .0,
            value
        );
    }
}

fn test_palette_len<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    assert!(palette.is_empty());
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value: value as u32,
            count: 1,
        });
        assert_eq!(palette.len(), value + 1);
    }
}

fn test_palette_index_size<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    assert_eq!(palette.index_size(), 0);
    palette.insert_new(PaletteEntry { value: 0, count: 1 });
    assert_eq!(palette.index_size(), 0);
    for value in 2..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value: value as u32,
            count: 1,
        });
        assert_eq!(palette.index_size(), calculate_smallest_index_size(value));
    }
}

fn test_pallete_get_by_value<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    for value in 0..amount_unique_inserts as u32 {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for value in 0..amount_unique_inserts as u32 {
        /*assert_eq!(
            palette.get_by_value(&value).map(|x| x.0),
            Some(&PaletteEntry { value, count: 1 })
        );*/
        assert_eq!(
            palette.get_mut_by_value(&value).map(|x| x.0),
            Some(&mut PaletteEntry { value, count: 1 })
        );
    }
}

fn test_palette_get_by_index<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    for value in 0..amount_unique_inserts as u32 {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for index in 0..amount_unique_inserts {
        assert_eq!(
            palette.get_by_index(index),
            Some(&PaletteEntry {
                value: index as u32,
                count: 1
            })
        );
        assert_eq!(
            palette.get_mut_by_index(index),
            Some(&mut PaletteEntry {
                value: index as u32,
                count: 1
            })
        );
    }
}

fn test_palette_mark_as_unused<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    for value in 0..amount_unique_inserts as u32 {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    for index in 0..amount_unique_inserts {
        palette.get_mut_by_index(index).unwrap().count = 0;
        palette.mark_as_unused(index);
    }
}

fn test_palette_mark_as_unused_len<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    for value in 0..amount_unique_inserts as u32 {
        palette.insert_new(PaletteEntry { value, count: 1 });
    }

    assert_eq!(palette.len(), amount_unique_inserts as usize);

    for index in 0..amount_unique_inserts {
        palette.get_mut_by_index(index).unwrap().count = 0;
        palette.mark_as_unused(index);
        assert_eq!(
            palette.len(),
            amount_unique_inserts as usize - index as usize - 1
        );
    }
    assert_eq!(palette.len(), 0);
}

fn test_palette_optimize<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    let mut control = Vec::new();
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value: value as u32,
            count: value as u32 + 1,
        });
        control.push(PaletteEntry {
            value: value as u32,
            count: value as u32 + 1,
        });
    }

    let old_len = palette.len();
    palette.optimize();
    assert_eq!(palette.len(), old_len);
    for i in (0..palette.len() as u32).step_by(2) {
        let (entry, index) = palette.get_mut_by_value(&i).unwrap();
        entry.count = 0;
        palette.mark_as_unused(index);
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
                .0
                .count,
            control_value.count
        );
    }
}

fn test_palette_index_size_after_optimizing<P: Palette<u32>>(
    mut palette: P,
    amount_unique_inserts: usize,
) {
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value: value as u32,
            count: value as u32 + 1,
        });
    }
    assert_eq!(
        palette.index_size(),
        calculate_smallest_index_size(amount_unique_inserts)
    );

    for i in 0..amount_unique_inserts {
        palette.get_mut_by_index(0).unwrap().count = 0;
        palette.mark_as_unused(0);
        palette.optimize();
        assert_eq!(
            palette.index_size(),
            calculate_smallest_index_size(amount_unique_inserts - i - 1)
        )
    }
}

fn test_palette_iter<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    assert!(palette.is_empty());
    let mut control = FxHashMap::default();
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value: value as u32,
            count: value as u32 + 1,
        });
        control.insert(value as u32, value as u32 + 1);
    }
    for entry in palette.iter() {
        assert_eq!(control.remove(&entry.value).unwrap(), entry.count);
    }
    assert!(control.is_empty());
}

fn test_palette_iter_mut<P: Palette<u32>>(mut palette: P, amount_unique_inserts: usize) {
    assert!(palette.is_empty());
    let mut control = FxHashMap::default();
    for value in 0..amount_unique_inserts {
        palette.insert_new(PaletteEntry {
            value: value as u32,
            count: value as u32 + 1,
        });
        control.insert(value as u32, value as u32 + 1);
    }
    for entry in palette.iter_mut() {
        assert_eq!(control.remove(&entry.value).unwrap(), entry.count);
        entry.count = 0;
    }
    assert!(control.is_empty());
}
