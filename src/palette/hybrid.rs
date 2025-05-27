use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use super::{Palette, PaletteEntry};

/// If the palette has less or equal to `INLINE_PALETTE_THRESHOLD` entries, it is stored on the stack.
const INLINE_PALETTE_THRESHOLD: usize = 32;

enum HybridPalette<T: Eq + Hash + Clone> {
    Array([Option<PaletteEntry<T>>; INLINE_PALETTE_THRESHOLD]),
    HashMap {
        index_map: HashMap<u64, PaletteEntry<T>>,
        value_map: HashMap<T, u64>,
    },
}

impl<T: Eq + Hash + Clone> HybridPalette<T> {
    pub fn new() -> Self {
        Self::Array([const { None }; INLINE_PALETTE_THRESHOLD])
    }

    fn switch_to_hashmap(&mut self) {
        match self {
            HybridPalette::HashMap { .. } => unreachable!(),
            HybridPalette::Array(array) => {
                let mut index_map = HashMap::new();
                let mut value_map = HashMap::new();
                for (i, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        value_map.insert(entry.value.clone(), i as u64);
                        index_map.insert(i as u64, entry);
                    }
                }
            }
        }
    }
}

impl<T: Eq + Hash + Clone> Palette<T> for HybridPalette<T> {
    fn get_by_value(&self, value: &T) -> Option<&PaletteEntry<T>> {
        match self {
            HybridPalette::Array(array) => {
                for entry in array.iter() {
                    if let Some(entry) = entry {
                        if &entry.value == value {
                            return Some(entry);
                        }
                    }
                }
                return None;
            }
            HybridPalette::HashMap {
                index_map,
                value_map,
            } => {
                let Some(index) = value_map.get(value) else {
                    return None;
                };
                return index_map.get(index);
            }
        }
    }

    fn get_mut_by_value(&mut self, value: &T) -> Option<&mut PaletteEntry<T>> {
        match self {
            HybridPalette::Array(array) => {
                for entry in array.iter_mut() {
                    if let Some(entry) = entry {
                        if &entry.value == value {
                            return Some(entry);
                        }
                    }
                }
                return None;
            }
            HybridPalette::HashMap {
                index_map,
                value_map,
            } => {
                let Some(index) = value_map.get(value) else {
                    return None;
                };
                return index_map.get_mut(index);
            }
        }
    }

    fn get_by_index(&self, index: u64) -> Option<&PaletteEntry<T>> {
        match self {
            HybridPalette::Array(array) => array[index as usize].as_ref(),
            HybridPalette::HashMap { index_map, .. } => index_map.get(&index),
        }
    }

    fn get_mut_by_index(&mut self, index: u64) -> Option<&mut PaletteEntry<T>> {
        match self {
            HybridPalette::Array(array) => array[index as usize].as_mut(),
            HybridPalette::HashMap { index_map, .. } => index_map.get_mut(&index),
        }
    }

    fn insert_new(&mut self, entry: PaletteEntry<T>) -> u64 {
        match self {
            HybridPalette::Array(array) => {
                // Try to use free spot
                for (i, old_entry) in array.iter_mut().enumerate() {
                    if old_entry.is_none() || old_entry.as_ref().unwrap().count == 0 {
                        *old_entry = Some(entry);
                        return i as u64;
                    }
                }
                // No free spot available, need to swich to hashmap
            }
            HybridPalette::HashMap {
                index_map,
                value_map,
            } => todo!(),
        }
    }

    fn optimize(&mut self) -> Option<HashMap<u64, T>> {
        todo!()
    }
}
