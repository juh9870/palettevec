use std::collections::HashMap;

use super::IndexBuffer;

pub struct AlignedIndexBuffer {
    index_size: usize,
    len: usize,
    storage: Vec<u64>,
}

impl AlignedIndexBuffer {
    pub fn new() -> Self {
        Self {
            index_size: 0,
            len: 0,
            storage: Vec::new(),
        }
    }

    fn set_index_with_index_size(&mut self, offset: usize, index_size: usize, index: u64) {
        let indices_per_u64 = 64 / index_size as usize;
        let target_u64 = &mut self.storage[offset / indices_per_u64];
        let target_offset = 64 - (offset % indices_per_u64 + 1) * index_size;
        let mask = (1 << index_size) - 1;
        *target_u64 &= !(mask << target_offset);
        *target_u64 |= (index as u64) << target_offset;
    }

    fn get_index_with_index_size(&self, offset: usize, index_size: usize) -> u64 {
        let indices_per_u64 = 64 / index_size as usize;
        let target_u64 = &self.storage[offset / indices_per_u64];
        let target_offset = 64 - (offset % indices_per_u64 + 1) * index_size;
        let mask = (1 << index_size) - 1;
        (*target_u64 >> target_offset) & mask
    }
}

impl IndexBuffer for AlignedIndexBuffer {
    fn set_index_size(&mut self, new_size: usize, new_mapping: Option<HashMap<u64, u64>>) {
        if new_size > self.index_size {
            // Index size grew, grow storage if needed and adjust indices
            let new_indices_per_u64 = 64 / new_size as usize;
            let needed_u64 = (self.len + new_indices_per_u64 - 1) / new_indices_per_u64; // ceil
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

    fn push_index(&mut self, index: u64) {
        let indices_per_u64 = 64 / self.index_size as usize;

        // Check if we need a new storage u64
        if self.len % indices_per_u64 == 0 {
            self.storage.push(index << (64 - self.index_size));
            self.len += 1;
            return;
        }

        // We can fit it into the last storage u64
        self.len += 1;
        self.set_index(self.len - 1, index);
    }

    fn pop_index(&mut self) -> Option<u64> {
        if self.len == 0 {
            return None;
        }
        let indices_per_u64 = 64 / self.index_size as usize;

        let index = self.get_index(self.len - 1);
        self.len -= 1;

        // Check if it's the last index in the storage u64
        if self.len % indices_per_u64 == 0 {
            self.storage.pop();
        }

        Some(index)
    }

    fn set_index(&mut self, offset: usize, index: u64) {
        self.set_index_with_index_size(offset, self.index_size, index);
    }

    fn get_index(&self, offset: usize) -> u64 {
        self.get_index_with_index_size(offset, self.index_size)
    }
}
