//! An `IndexBuffer` implementation that stores indices
//! packed tightly into a `Vec<u64>`.
//!
//! It does NOT store u64-boundary crossing indices. This means slightly more
//! memory usage for slightly faster access times. This is a good default.

use rustc_hash::FxHashMap;

use crate::MemoryUsage;
use crate::palette::CountType;
use super::IndexBuffer;

/// An `IndexBuffer` implementation that stores indices
/// packed tightly into a `Vec<u64>`.
///
/// It does NOT store u64-boundary crossing indices. This means slightly more
/// memory usage for slightly faster access times. This is a good default.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct AlignedIndexBuffer {
    index_size: usize,
    indices_per_u64: u8,
    mask: u64,
    len: usize,
    storage: Vec<u64>,
}

impl AlignedIndexBuffer {
    fn set_index_with_index_size(
        &mut self,
        offset: usize,
        index_size: usize,
        indices_per_u64: usize,
        index: usize,
    ) -> usize {
        debug_assert!(index_size > 0);
        debug_assert_eq!(64 / index_size, indices_per_u64);
        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked_mut(offset / indices_per_u64) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &mut self.storage[offset / indices_per_u64]
            }
        };
        let target_offset = (offset % indices_per_u64) * index_size;
        let mask = u64::MAX >> (64 - index_size);
        let old_index = (*target_u64 >> target_offset) & mask;
        *target_u64 &= !(mask << target_offset);
        *target_u64 |= (index as u64) << target_offset;
        old_index as usize
    }

    fn _set_index(&mut self, offset: usize, index: usize) -> usize {
        let indices_per_u64 = self.indices_per_u64 as usize;
        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked_mut(offset / indices_per_u64) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &mut self.storage[offset / indices_per_u64]
            }
        };
        let target_offset = (offset % indices_per_u64) * self.index_size;
        let old_index = (*target_u64 >> target_offset) & self.mask;
        *target_u64 &= !(self.mask << target_offset);
        *target_u64 |= (index as u64) << target_offset;
        old_index as usize
    }

    fn _get_index(&self, offset: usize) -> usize {
        if self.index_size == 0 {
            return 0;
        }
        let indices_per_u64 = self.indices_per_u64 as usize;
        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked(offset / indices_per_u64) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &self.storage[offset / indices_per_u64]
            }
        };
        let target_offset = (offset % indices_per_u64) * self.index_size;
        ((*target_u64 >> target_offset) & self.mask) as usize
    }
}

impl IndexBuffer for AlignedIndexBuffer {
    fn new() -> Self {
        Self {
            index_size: 0,
            indices_per_u64: 0,
            len: 0,
            mask: 0,
            storage: Vec::new(),
        }
    }

    fn zeroed(&mut self, len: usize) {
        if self.index_size == 0 {
            debug_assert!(self.storage.is_empty());
            self.len = len;
            return;
        }
        let indices_per_u64 = 64 / self.index_size;
        self.indices_per_u64 = indices_per_u64 as u8;
        let needed_u64 = len.div_ceil(indices_per_u64);
        self.mask = (1 << self.index_size) - 1;
        self.storage.resize(needed_u64, 0);
        self.storage.fill(0);
        self.len = len;
    }

    fn clear(&mut self) {
        self.index_size = 0;
        self.indices_per_u64 = 0;
        self.mask = 0;
        self.len = 0;
        self.storage.clear();
    }
    
    fn resize(&mut self, new_len: usize, index: usize) -> (Option<FxHashMap<usize, CountType>>, Option<CountType>) {
        if self.len == 0 && index == 0 {
            self.zeroed(new_len);
            return (None, Some(new_len as CountType))
        }
        if new_len < self.len {
            let mut removed_indices = FxHashMap::default();
            while new_len < self.len {
                if let Some(idx) = self.pop_index() {
                    removed_indices.entry(idx).and_modify(|e| *e += 1).or_insert(1);
                }
            }
            return (Some(removed_indices), None);
        } else if new_len > self.len {
            let added = new_len - self.len;
            while new_len > self.len {
                self.push_index(index);
            }
            return (None, Some(added as CountType));
        }
        (None, None)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn memory_usage(&self) -> MemoryUsage {
        MemoryUsage {
            stack: std::mem::size_of::<Self>(),
            heap_actually_needed: self.storage.len() * std::mem::size_of::<u64>(),
            heap_allocated: self.storage.capacity() * std::mem::size_of::<u64>(),
        }
    }

    fn set_index_size(&mut self, new_size: usize, new_mapping: Option<FxHashMap<usize, usize>>) {
        if new_size > self.index_size {
            // Index size grew, grow storage if needed and adjust indices
            let new_indices_per_u64 = 64 / new_size;
            let needed_u64 = self.len.div_ceil(new_indices_per_u64);
            self.storage.resize(needed_u64, 0);
            if let Some(mapping) = new_mapping {
                // Mapping provided, adjust indices
                // We can work inplace by starting from the end and going backwards
                for i in (0..self.len).rev() {
                    let old_index = self.get_index(i);
                    let new_index = mapping.get(&old_index).unwrap();
                    // We can just override the storage in place because we go backwards
                    self.set_index_with_index_size(i, new_size, new_indices_per_u64, *new_index);
                }
            } else {
                // No mapping provided, adjust indices without mapping
                // We can work inplace by starting from the end and going backwards
                for i in (0..self.len).rev() {
                    let old_index = self.get_index(i);
                    self.set_index_with_index_size(i, new_size, new_indices_per_u64, old_index);
                }
            }
            self.indices_per_u64 = new_indices_per_u64 as u8;
            self.mask = (1 << new_size) - 1;
        } else if new_size < self.index_size {
            if new_size == 0 {
                if let Some(new_mapping) = new_mapping {
                    debug_assert!(new_mapping.len() == 1);
                    debug_assert!(new_mapping.values().any(|x| *x == 0));
                }
                self.index_size = 0;
                self.storage.clear();
                return;
            }
            let new_indices_per_u64 = 64 / new_size;
            // Index size shrinked, keep storage size and adjust indices
            if let Some(mapping) = new_mapping {
                // Mapping provided, adjust indices
                for i in 0..self.len {
                    let old_index = self.get_index(i);
                    let new_index = mapping.get(&old_index).unwrap();
                    // We can just override the storage in place because the new size is smaller
                    self.set_index_with_index_size(i, new_size, new_indices_per_u64, *new_index);
                }
            } else {
                // No Mapping provided, just truncate indices
                for i in 0..self.len {
                    let index = self.get_index(i);
                    // We can just override the storage in place because the new size is smaller
                    self.set_index_with_index_size(i, new_size, new_indices_per_u64, index);
                }
            }
            let new_indices_per_u64 = 64 / new_size;
            self.indices_per_u64 = new_indices_per_u64 as u8;
            self.mask = (1 << new_size) - 1;
            let needed_u64 = self.len.div_ceil(new_indices_per_u64);
            self.storage.truncate(needed_u64);
        } else if let Some(mapping) = new_mapping {
            // Index size stayed the same, apply new mapping if provided
            for i in 0..self.len {
                let old_index = self.get_index(i);
                let new_index = mapping.get(&old_index).unwrap();
                self.set_index(i, *new_index);
            }
        }
        self.index_size = new_size;
    }

    fn push_index(&mut self, index: usize) {
        if self.index_size == 0 {
            self.len += 1;
            return;
        }
        let indices_per_u64 = self.indices_per_u64 as usize;

        // Check if we need a new storage u64
        if self.len % indices_per_u64 == 0 {
            self.storage.push(index as u64);
            self.len += 1;
            return;
        }

        // We can fit it into the last storage u64
        self.len += 1;
        self.set_index(self.len - 1, index);
    }

    fn pop_index(&mut self) -> Option<usize> {
        if self.len == 0 {
            return None;
        }
        if self.index_size == 0 {
            self.len -= 1;
            return Some(0);
        }
        let indices_per_u64 = self.indices_per_u64 as usize;

        let index = self.get_index(self.len - 1);
        self.len -= 1;

        // Check if it's the last index in the storage u64
        if self.len % indices_per_u64 == 0 {
            self.storage.pop();
        }

        Some(index)
    }

    fn set_index(&mut self, offset: usize, index: usize) -> usize {
        debug_assert!(
            self.index_size > 0,
            "Handle set on index_size == 0 one abstraction level above please :)"
        );
        debug_assert!(offset < self.len);
        self._set_index(offset, index)
    }

    fn get_index(&self, offset: usize) -> usize {
        debug_assert!(offset < self.len);
        self._get_index(offset)
    }

    type Iter<'a>
        = AlignedIndexIterator<'a>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        AlignedIndexIterator {
            buffer: self,
            offset: 0,
        }
    }
}

// ITERATOR
#[derive(Debug, Clone)]
pub struct AlignedIndexIterator<'a> {
    buffer: &'a AlignedIndexBuffer,
    offset: usize,
}

impl<'a> Iterator for AlignedIndexIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.buffer.len {
            None
        } else {
            let index = self.buffer.get_index(self.offset);
            self.offset += 1;
            Some(index)
        }
    }
}
