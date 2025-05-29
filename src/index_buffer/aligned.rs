use std::collections::HashMap;

use super::IndexBuffer;

pub struct AlignedIndexBuffer {
    index_size: usize,
    len: usize,
    storage: Vec<u64>,
}

impl AlignedIndexBuffer {
    fn set_index_with_index_size(
        &mut self,
        offset: usize,
        index_size: usize,
        index: usize,
    ) -> usize {
        debug_assert!(index_size > 0);
        let indices_per_u64 = 64 / index_size;
        let target_u64 = &mut self.storage[offset / indices_per_u64];
        let target_offset = 64 - (offset % indices_per_u64 + 1) * index_size;
        let mask = (1 << index_size) - 1;
        let old_index = (*target_u64 >> target_offset) & mask;
        *target_u64 &= !(mask << target_offset);
        *target_u64 |= (index as u64) << target_offset;
        old_index as usize
    }

    fn get_index_with_index_size(&self, offset: usize, index_size: usize) -> usize {
        if index_size == 0 {
            return 0;
        }
        let indices_per_u64 = 64 / index_size;
        let target_u64 = &self.storage[offset / indices_per_u64];
        let target_offset = 64 - (offset % indices_per_u64 + 1) * index_size;
        let mask = (1 << index_size) - 1;
        ((*target_u64 >> target_offset) & mask) as usize
    }
}

impl IndexBuffer for AlignedIndexBuffer {
    fn new() -> Self {
        Self {
            index_size: 0,
            len: 0,
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
        let needed_u64 = len.div_ceil(indices_per_u64);
        self.storage.resize(needed_u64, 0);
        self.storage.fill(0);
        self.len = len;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn set_index_size(&mut self, new_size: usize, new_mapping: Option<HashMap<usize, usize>>) {
        if new_size > self.index_size {
            // Index size grew, grow storage if needed and adjust indices
            let new_indices_per_u64 = 64 / new_size;
            let needed_u64 = self.len.div_ceil(new_indices_per_u64);
            let have_u64 = self.storage.capacity();
            if have_u64 < needed_u64 {
                self.storage.reserve(needed_u64 - have_u64);
            }
            self.storage.resize(needed_u64, 0);
            if let Some(mapping) = new_mapping {
                // Mapping provided, adjust indices
                // We can work inplace by starting from the end and going backwards
                for i in (0..self.len).rev() {
                    let old_index = self.get_index(i);
                    let new_index = mapping.get(&old_index).unwrap();
                    // We can just override the storage in place because we go backwards
                    self.set_index_with_index_size(i, new_size, *new_index);
                }
            } else {
                // No mapping provided, adjust indices without mapping
                // We can work inplace by starting from the end and going backwards
                for i in (0..self.len).rev() {
                    let old_index = self.get_index(i);
                    self.set_index_with_index_size(i, new_size, old_index);
                }
            }
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
            // Index size shrinked, keep storage size and adjust indices
            if let Some(mapping) = new_mapping {
                // Mapping provided, adjust indices
                for i in 0..self.len {
                    let old_index = self.get_index(i);
                    let new_index = mapping.get(&old_index).unwrap();
                    // We can just override the storage in place because the new size is smaller
                    self.set_index_with_index_size(i, new_size, *new_index);
                }
            } else {
                // No Mapping provided, just truncate indices
                for i in 0..self.len {
                    let index = self.get_index(i);
                    // We can just override the storage in place because the new size is smaller
                    self.set_index_with_index_size(i, new_size, index);
                }
            }
            let new_indices_per_u64 = 64 / new_size;
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
        let indices_per_u64 = 64 / self.index_size;

        // Check if we need a new storage u64
        if self.len % indices_per_u64 == 0 {
            self.storage.push((index as u64) << (64 - self.index_size));
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
        let indices_per_u64 = 64 / self.index_size;

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
        self.set_index_with_index_size(offset, self.index_size, index)
    }

    fn get_index(&self, offset: usize) -> usize {
        debug_assert!(offset < self.len);
        self.get_index_with_index_size(offset, self.index_size)
    }
}
