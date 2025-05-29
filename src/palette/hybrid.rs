use std::hash::Hash;

use rustc_hash::FxHashMap;

use crate::palette::{calculate_smallest_index_size, compare_palette_entries_max_first};

use super::{compare_palette_entries_option_max_first, Palette, PaletteEntry};

pub struct HybridPalette<const INLINE_PALETTE_THRESHOLD: usize, T: Eq + Hash + Clone> {
    index_size: usize,
    real_entries: usize,
    storage: HybridStorage<INLINE_PALETTE_THRESHOLD, T>,
}

enum HybridStorage<const INLINE_PALETTE_THRESHOLD: usize, T: Eq + Hash + Clone> {
    Array {
        array: [Option<PaletteEntry<T>>; INLINE_PALETTE_THRESHOLD],
    },
    HashMap {
        free_indices: Vec<usize>,
        index_map: FxHashMap<usize, PaletteEntry<T>>,
        value_map: FxHashMap<T, usize>,
    },
}

impl<const INLINE_PALETTE_THRESHOLD: usize, T: Eq + Hash + Clone>
    HybridPalette<INLINE_PALETTE_THRESHOLD, T>
{
    fn switch_to_hashmap(&mut self) {
        match &mut self.storage {
            HybridStorage::HashMap { .. } => unreachable!(),
            HybridStorage::Array { array } => {
                let mut free_indices = Vec::new();
                let mut index_map = FxHashMap::default();
                let mut value_map = FxHashMap::default();
                for (i, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        debug_assert!(entry.count > 0);
                        value_map.insert(entry.value.clone(), i);
                        index_map.insert(i, entry.clone());
                    } else {
                        free_indices.push(i);
                    }
                }
                debug_assert_eq!(index_map.len(), value_map.len());
                // Make sure the array is full before switching to hashmap
                debug_assert_eq!(index_map.len(), INLINE_PALETTE_THRESHOLD);
                self.storage = HybridStorage::HashMap {
                    free_indices,
                    index_map,
                    value_map,
                };
            }
        }
    }

    /// Returns mapping of old_indices to new_indices if necessary
    fn switch_to_array(&mut self) -> Option<FxHashMap<usize, usize>> {
        match &mut self.storage {
            HybridStorage::Array { .. } => unreachable!(),
            HybridStorage::HashMap {
                index_map,
                value_map,
                ..
            } => {
                debug_assert_eq!(index_map.len(), value_map.len());
                debug_assert!(index_map.len() <= INLINE_PALETTE_THRESHOLD);
                let mut new_mapping = FxHashMap::default();
                let mut array: [Option<PaletteEntry<T>>; INLINE_PALETTE_THRESHOLD] =
                    [const { None }; INLINE_PALETTE_THRESHOLD];

                let mut needs_new_mapping = false;
                let mut index_map = index_map.iter().collect::<Vec<_>>();
                index_map.sort_by(|a, b| {
                    // .then_with to break ties for deterministic testing
                    compare_palette_entries_max_first(a.1, b.1).then_with(|| a.0.cmp(b.0))
                });
                for (new_index, (old_index, entry)) in index_map.iter().enumerate() {
                    debug_assert!(entry.count > 0);
                    if new_index != **old_index {
                        needs_new_mapping = true;
                    }
                    new_mapping.insert(**old_index, new_index);
                    array[new_index] = Some((*entry).clone());
                }

                self.storage = HybridStorage::Array { array };

                if needs_new_mapping {
                    return Some(new_mapping);
                }
                None
            }
        }
    }
}

impl<const INLINE_PALETTE_THRESHOLD: usize, T: Eq + Hash + Clone> Palette<T>
    for HybridPalette<INLINE_PALETTE_THRESHOLD, T>
{
    fn new() -> Self {
        Self {
            index_size: 0,
            real_entries: 0,
            storage: HybridStorage::Array {
                array: [const { None }; INLINE_PALETTE_THRESHOLD],
            },
        }
    }

    fn len(&self) -> usize {
        self.real_entries
    }

    fn is_empty(&self) -> bool {
        self.real_entries == 0
    }

    fn index_size(&self) -> usize {
        self.index_size
    }

    fn mark_as_unused(&mut self, index: usize) {
        self.real_entries -= 1;
        match &mut self.storage {
            HybridStorage::Array { array, .. } => {
                array[index] = None;
            }
            HybridStorage::HashMap {
                free_indices,
                index_map,
                value_map,
            } => {
                free_indices.push(index);
                let entry = index_map.remove(&index).unwrap();
                debug_assert_eq!(entry.count, 0);
                value_map.remove(&entry.value);
            }
        }
    }

    /*fn get_by_value(&self, value: &T) -> Option<(&PaletteEntry<T>, usize)> {
        match &self.storage {
            HybridStorage::Array { array, .. } => {
                for (index, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        if &entry.value == value {
                            return Some((entry, index));
                        }
                    }
                }
                None
            }
            HybridStorage::HashMap {
                index_map,
                value_map,
                ..
            } => {
                let index = value_map.get(value)?;
                index_map.get(index).map(|entry| (entry, *index))
            }
        }
    }*/

    fn get_mut_by_value(&mut self, value: &T) -> Option<(&mut PaletteEntry<T>, usize)> {
        match &mut self.storage {
            HybridStorage::Array { array, .. } => {
                for (index, entry) in array.iter_mut().enumerate() {
                    if let Some(entry) = entry {
                        if &entry.value == value {
                            return Some((entry, index));
                        }
                    }
                }
                None
            }
            HybridStorage::HashMap {
                index_map,
                value_map,
                ..
            } => {
                let index = value_map.get(value)?;
                index_map.get_mut(index).map(|entry| (entry, *index))
            }
        }
    }

    fn get_by_index(&self, index: usize) -> Option<&PaletteEntry<T>> {
        match &self.storage {
            HybridStorage::Array { array, .. } => array[index].as_ref(),
            HybridStorage::HashMap { index_map, .. } => index_map.get(&index),
        }
    }

    fn get_mut_by_index(&mut self, index: usize) -> Option<&mut PaletteEntry<T>> {
        match &mut self.storage {
            HybridStorage::Array { array, .. } => array[index].as_mut(),
            HybridStorage::HashMap { index_map, .. } => index_map.get_mut(&index),
        }
    }

    fn insert_new(&mut self, entry: PaletteEntry<T>) -> (usize, Option<usize>) {
        debug_assert!(entry.count > 0);
        match &mut self.storage {
            HybridStorage::Array { array, .. } => {
                // Try to use free spot
                for (i, old_entry) in array.iter_mut().enumerate() {
                    if old_entry.is_none() || old_entry.as_ref().unwrap().count == 0 {
                        *old_entry = Some(entry);
                        self.real_entries += 1;
                        let new_index_size = calculate_smallest_index_size(self.real_entries);
                        let mut actual_new_index_size = None;
                        if new_index_size > self.index_size {
                            self.index_size = new_index_size;
                            actual_new_index_size = Some(new_index_size);
                        }
                        return (i, actual_new_index_size);
                    }
                }
                // No free spot available, need to swich to hashmap
                self.switch_to_hashmap();
                self.insert_new(entry)
            }
            HybridStorage::HashMap {
                free_indices,
                index_map,
                value_map,
            } => {
                // Check if free index is available
                if let Some(index) = free_indices.pop() {
                    value_map.insert(entry.value.clone(), index);
                    index_map.insert(index, entry);
                    self.real_entries += 1;
                    return (index, None);
                }

                // No free index available, create a new one
                let index = index_map.len();
                value_map.insert(entry.value.clone(), index);
                index_map.insert(index, entry);
                self.real_entries += 1;
                let new_index_size = calculate_smallest_index_size(self.real_entries);
                let mut actual_new_index_size = None;
                if new_index_size > self.index_size {
                    self.index_size = new_index_size;
                    actual_new_index_size = Some(new_index_size);
                }
                (index, actual_new_index_size)
            }
        }
    }

    fn optimize(&mut self) -> Option<FxHashMap<usize, usize>> {
        self.index_size = calculate_smallest_index_size(self.real_entries);
        match &mut self.storage {
            HybridStorage::Array { array, .. } => {
                // To optimize the array palette version, we sort palette
                // entries by their size. Max count first.

                // Save old mapping
                let mut old_mapping = FxHashMap::default();
                for (i, entry) in array.iter().enumerate() {
                    if let Some(entry) = entry {
                        old_mapping.insert(entry.value.clone(), i);
                    }
                }

                // Sort the array
                array.sort_by(compare_palette_entries_option_max_first);

                // Create new mapping
                let mut needs_new_mapping = false;
                let mut new_mapping = FxHashMap::default();
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
                    new_mapping.insert(*old_index, new_index);
                }
                if needs_new_mapping {
                    return Some(new_mapping);
                }
                None
            }
            HybridStorage::HashMap {
                free_indices,
                index_map,
                value_map,
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
                let mut new_mapping = FxHashMap::default();
                let mut new_index_map = FxHashMap::default();
                let mut new_value_map = FxHashMap::default();

                let mut entries = index_map.drain().collect::<Vec<_>>();
                entries.sort_by(|a, b| {
                    // .then_with to break ties for deterministic testing
                    compare_palette_entries_max_first(&a.1, &b.1).then_with(|| a.0.cmp(&b.0))
                });

                for (new_index, (old_index, entry)) in entries.into_iter().enumerate() {
                    new_value_map.insert(entry.value.clone(), new_index);
                    new_index_map.insert(new_index, entry);
                    new_mapping.insert(old_index, new_index);
                }
                free_indices.clear();
                self.storage = HybridStorage::HashMap {
                    free_indices: std::mem::take(free_indices),
                    index_map: new_index_map,
                    value_map: new_value_map,
                };
                Some(new_mapping)
            }
        }
    }
}
