use crate::Palette;
use std::{hash::Hash, iter::FilterMap};

use rustc_hash::FxHashMap;

use crate::{
    palette::{
        calculate_smallest_index_size, compare_palette_entries_option_max_first, PaletteEntry,
    },
    MemoryUsage,
};

/// A Palette based purely on a heap allocated vec.
///
/// Not optimal for very large palette sizes, but very fast for small ones.
///
/// Also very memory efficient and no danger of stack overflow.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct VecPalette<T: Eq + Hash + Clone> {
    index_size: usize,
    real_entries: usize,
    storage: Vec<Option<PaletteEntry<T>>>,
}

impl<T: Eq + Hash + Clone> Palette<T> for VecPalette<T> {
    fn new() -> Self {
        VecPalette {
            index_size: 0,
            real_entries: 0,
            storage: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.real_entries
    }

    fn is_empty(&self) -> bool {
        self.real_entries == 0
    }

    fn memory_usage(&self) -> MemoryUsage {
        MemoryUsage {
            stack: std::mem::size_of::<Self>(),
            heap_actually_needed: self.storage.len() * std::mem::size_of::<PaletteEntry<T>>(),
            heap_allocated: self.storage.capacity() * std::mem::size_of::<PaletteEntry<T>>(),
        }
    }

    fn index_size(&self) -> usize {
        self.index_size
    }
    
    fn clear(&mut self) { 
        self.storage.clear();
        self.index_size = 0;
        self.real_entries = 0;
    }
    
    fn mark_as_unused(&mut self, index: usize) {
        debug_assert!(self.storage[index].is_some());
        self.real_entries -= 1;
        self.storage[index] = None;
    }

    fn get_mut_by_value(&mut self, value: &T) -> Option<(&mut PaletteEntry<T>, usize)> {
        for (index, entry) in self.storage.iter_mut().enumerate() {
            if let Some(entry) = entry {
                if &entry.value == value {
                    return Some((entry, index));
                }
            }
        }
        None
    }

    fn get_by_index(&self, index: usize) -> Option<&PaletteEntry<T>> {
        self.storage[index].as_ref()
    }

    fn get_mut_by_index(&mut self, index: usize) -> Option<&mut PaletteEntry<T>> {
        self.storage[index].as_mut()
    }

    fn insert_new(&mut self, entry: PaletteEntry<T>) -> (usize, Option<usize>) {
        debug_assert!(entry.count > 0);
        // Try to use free spot
        for (i, old_entry) in self.storage.iter_mut().enumerate() {
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

        // No free slot, just push new
        let index = self.storage.len();
        self.storage.push(Some(entry));
        self.real_entries += 1;
        let new_index_size = calculate_smallest_index_size(self.real_entries);
        let mut actual_new_index_size = None;
        if new_index_size > self.index_size {
            self.index_size = new_index_size;
            actual_new_index_size = Some(new_index_size);
        }
        (index, actual_new_index_size)
    }

    fn optimize(&mut self) -> Option<FxHashMap<usize, usize>> {
        self.index_size = calculate_smallest_index_size(self.real_entries);
        // To optimize the vec palette, we sort palette
        // entries by their size. Max count first.

        // Save old mapping
        let mut old_mapping = FxHashMap::default();
        for (i, entry) in self.storage.iter().enumerate() {
            if let Some(entry) = entry {
                old_mapping.insert(entry.value.clone(), i);
            }
        }

        // Sort the array
        self.storage
            .sort_by(compare_palette_entries_option_max_first);

        // Create new mapping
        let mut needs_new_mapping = false;
        let mut new_mapping = FxHashMap::default();
        for (new_index, entry) in self.storage.iter().enumerate() {
            let Some(entry) = entry else {
                // Early break is allowed because we sort the vec
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

    type EntriesIter<'a>
        = VecPaletteEntriesIter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn iter(&self) -> Self::EntriesIter<'_> {
        VecPaletteEntriesIter {
            data: self.storage.iter().filter_map(Option::as_ref),
        }
    }

    type EntriesIterMut<'a>
        = VecPaletteEntriesIterMut<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn iter_mut(&mut self) -> Self::EntriesIterMut<'_> {
        VecPaletteEntriesIterMut {
            data: self.storage.iter_mut().filter_map(Option::as_mut),
        }
    }
}

// REF ITERATOR for VecPalette
type VecPaletteEntriesFilter<'a, T> = FilterMap<
    std::slice::Iter<'a, Option<PaletteEntry<T>>>,
    fn(&'a Option<PaletteEntry<T>>) -> Option<&'a PaletteEntry<T>>,
>;

#[derive(Debug, Clone)]
pub struct VecPaletteEntriesIter<'a, T: Eq + Clone + 'a> {
    data: VecPaletteEntriesFilter<'a, T>,
}

impl<'a, T: Eq + Clone> Iterator for VecPaletteEntriesIter<'a, T> {
    type Item = &'a PaletteEntry<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data.size_hint()
    }
}

// MUTABLE ITERATOR for VecPalette
type VecPaletteEntriesFilterMut<'a, T> = FilterMap<
    std::slice::IterMut<'a, Option<PaletteEntry<T>>>,
    fn(&'a mut Option<PaletteEntry<T>>) -> Option<&'a mut PaletteEntry<T>>,
>;

#[derive(Debug)]
pub struct VecPaletteEntriesIterMut<'a, T: Eq + Clone + 'a> {
    data: VecPaletteEntriesFilterMut<'a, T>,
}

impl<'a, T: Eq + Clone> Iterator for VecPaletteEntriesIterMut<'a, T> {
    type Item = &'a mut PaletteEntry<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data.size_hint()
    }
}
