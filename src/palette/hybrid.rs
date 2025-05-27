TODO: ADJUST INDEX SIZE ON INSERT_NEW AND OPTIMIZE

use std::{collections::HashMap, hash::Hash};

use crate::palette::compare_palette_entries_max_first;

use super::{compare_palette_entries_option_max_first, Palette, PaletteEntry};

/// If the palette has less or equal to `INLINE_PALETTE_THRESHOLD` entries, it is stored on the stack.
pub enum HybridPalette<T: Eq + Hash + Clone, const INLINE_PALETTE_THRESHOLD: usize> {
    Array {
        index_size: usize,
        array: [Option<PaletteEntry<T>>; INLINE_PALETTE_THRESHOLD],
    },
    HashMap {
        index_size: usize,
        free_indices: Vec<u64>,
        index_map: HashMap<u64, PaletteEntry<T>>,
        value_map: HashMap<T, u64>,
    },
}

impl<T: Eq + Hash + Clone, const INLINE_PALETTE_THRESHOLD: usize>
    HybridPalette<T, INLINE_PALETTE_THRESHOLD>
{
    pub fn new() -> Self {
        Self::Array {
            index_size: 0,
            array: [const { None }; INLINE_PALETTE_THRESHOLD],
        }
    }

    fn switch_to_hashmap(&mut self) {
        match self {
            HybridPalette::HashMap { .. } => unreachable!(),
            HybridPalette::Array { array, index_size } => {
                let mut free_indices = Vec::new();
                let mut index_map = HashMap::new();
                let mut value_map = HashMap::new();
                for (i, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        value_map.insert(entry.value.clone(), i as u64);
                        index_map.insert(i as u64, entry.clone());
                    } else {
                        free_indices.push(i as u64);
                    }
                }
                debug_assert_eq!(index_map.len(), value_map.len());
                // Make sure the array is full before switching to hashmap
                debug_assert_eq!(index_map.len(), INLINE_PALETTE_THRESHOLD);

                *self = HybridPalette::HashMap {
                    index_size: *index_size,
                    free_indices,
                    index_map,
                    value_map,
                };
            }
        }
    }

    /// Returns mapping of old_indices to new_indices if necessary
    fn switch_to_array(&mut self) -> Option<HashMap<u64, u64>> {
        match self {
            HybridPalette::Array { .. } => unreachable!(),
            HybridPalette::HashMap {
                index_size,
                index_map,
                value_map,
                ..
            } => {
                debug_assert_eq!(index_map.len(), value_map.len());
                debug_assert!(index_map.len() <= INLINE_PALETTE_THRESHOLD);
                let mut new_mapping = HashMap::new();
                let mut array: [Option<PaletteEntry<T>>; INLINE_PALETTE_THRESHOLD] =
                    [const { None }; INLINE_PALETTE_THRESHOLD];

                let mut needs_new_mapping = false;
                for (new_index, (old_index, entry)) in index_map.iter().enumerate() {
                    debug_assert!(entry.count > 0);
                    if new_index as u64 != *old_index {
                        needs_new_mapping = true;
                    }
                    new_mapping.insert(*old_index, new_index as u64);
                    array[new_index] = Some(entry.clone());
                }
                if !needs_new_mapping {
                    let unsorted_array = array.clone();
                    array.sort_by(compare_palette_entries_option_max_first);
                    if array != unsorted_array {
                        needs_new_mapping = true;
                    }
                }

                *self = HybridPalette::Array {
                    index_size: *index_size,
                    array,
                };

                if needs_new_mapping {
                    return Some(new_mapping);
                }
                None
            }
        }
    }
}

impl<T: Eq + Hash + Clone, const INLINE_PALETTE_THRESHOLD: usize> Palette<T>
    for HybridPalette<T, INLINE_PALETTE_THRESHOLD>
{
    fn len(&self) -> usize {
        match self {
            HybridPalette::Array { array, .. } => {
                let mut len = 0;
                for entry in array.iter() {
                    if let Some(entry) = entry {
                        if entry.count > 0 {
                            len += 1;
                        }
                    }
                }
                len
            }
            HybridPalette::HashMap { index_map, .. } => index_map.len(),
        }
    }

    fn index_size(&self) -> usize {
        match self {
            HybridPalette::Array { index_size, .. } => *index_size,
            HybridPalette::HashMap { index_size, .. } => *index_size,
        }
    }

    fn mark_as_unused(&mut self, index: u64) {
        match self {
            HybridPalette::Array { .. } => return,
            HybridPalette::HashMap {
                free_indices,
                index_map,
                value_map,
                ..
            } => {
                free_indices.push(index);
                let entry = index_map.remove(&index).unwrap();
                debug_assert_eq!(entry.count, 0);
                value_map.remove(&entry.value);
            }
        }
    }

    fn get_by_value(&self, value: &T) -> Option<&PaletteEntry<T>> {
        match self {
            HybridPalette::Array { array, .. } => {
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
                ..
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
            HybridPalette::Array { array, .. } => {
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
                ..
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
            HybridPalette::Array { array, .. } => array[index as usize].as_ref(),
            HybridPalette::HashMap { index_map, .. } => index_map.get(&index),
        }
    }

    fn get_mut_by_index(&mut self, index: u64) -> Option<&mut PaletteEntry<T>> {
        match self {
            HybridPalette::Array { array, .. } => array[index as usize].as_mut(),
            HybridPalette::HashMap { index_map, .. } => index_map.get_mut(&index),
        }
    }

    fn get_index_from_value(&self, value: &T) -> Option<u64> {
        match self {
            HybridPalette::Array { array, .. } => {
                for (i, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        if entry.value == *value {
                            return Some(i as u64);
                        }
                    }
                }
                None
            }
            HybridPalette::HashMap { value_map, .. } => value_map.get(value).copied(),
        }
    }

    fn insert_new(&mut self, entry: PaletteEntry<T>) -> u64 {
        debug_assert!(entry.count > 0);
        match self {
            HybridPalette::Array { array, .. } => {
                // Try to use free spot
                for (i, old_entry) in array.iter_mut().enumerate() {
                    if old_entry.is_none() || old_entry.as_ref().unwrap().count == 0 {
                        *old_entry = Some(entry);
                        return i as u64;
                    }
                }
                // No free spot available, need to swich to hashmap
                self.switch_to_hashmap();
                self.insert_new(entry)
            }
            HybridPalette::HashMap {
                free_indices,
                index_map,
                value_map,
                ..
            } => {
                // Check if free index is available
                if let Some(index) = free_indices.pop() {
                    value_map.insert(entry.value.clone(), index);
                    index_map.insert(index, entry);
                    return index;
                }

                // No free index available, create a new one
                let index = index_map.len() as u64;
                value_map.insert(entry.value.clone(), index);
                index_map.insert(index, entry);
                index
            }
        }
    }

    fn optimize(&mut self) -> Option<HashMap<u64, u64>> {
        match self {
            HybridPalette::Array { array, .. } => {
                // To optimize the array palette version, we sort palette
                // entries by their size. Max count first.

                // Save old mapping
                let mut old_mapping = HashMap::new();
                for (i, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        old_mapping.insert(entry.value.clone(), i);
                    }
                }

                // Sort the array
                array.sort_by(compare_palette_entries_option_max_first);

                // Create new mapping
                let mut needs_new_mapping = false;
                let mut new_mapping = HashMap::new();
                for (new_index, entry) in array.iter().enumerate() {
                    let Some(entry) = entry else {
                        // Early break is allowed because we sort the array
                        // so that Nones are piled together at the end.
                        break;
                    };
                    let old_index = old_mapping.get(&entry.value).unwrap();
                    if new_index != *old_index {
                        needs_new_mapping = true;
                    }
                    new_mapping.insert(*old_index as u64, new_index as u64);
                }
                if needs_new_mapping {
                    return Some(new_mapping);
                }
                None
            }
            HybridPalette::HashMap {
                free_indices,
                index_map,
                value_map,
                index_size,
            } => {
                debug_assert_eq!(index_map.len(), value_map.len());
                // If we can switch to array, prefer that
                if index_map.len() <= INLINE_PALETTE_THRESHOLD {
                    return self.switch_to_array();
                }

                // Is the hashmap already optimal?
                if free_indices.is_empty() {
                    return None;
                }

                // We can't switch, so lets pack the indices closer together
                let mut new_mapping = HashMap::new();
                let mut new_index_map = HashMap::new();
                let mut new_value_map = HashMap::new();

                let mut entries = index_map.drain().collect::<Vec<_>>();
                entries.sort_unstable_by(|a, b| compare_palette_entries_max_first(&a.1, &b.1));

                for (new_index, (old_index, entry)) in entries.into_iter().enumerate() {
                    new_value_map.insert(entry.value.clone(), new_index as u64);
                    new_index_map.insert(new_index as u64, entry);
                    new_mapping.insert(old_index as u64, new_index as u64);
                }
                free_indices.clear();
                *self = HybridPalette::HashMap {
                    free_indices: std::mem::take(free_indices),
                    index_map: new_index_map,
                    value_map: new_value_map,
                    index_size: *index_size,
                };
                return Some(new_mapping);
            }
        }
    }
}
