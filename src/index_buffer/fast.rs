use rustc_hash::FxHashMap;

use crate::MemoryUsage;
use crate::palette::CountType;
use super::IndexBuffer;

fn map_index_size(from_palette: usize) -> usize {
    debug_assert!(from_palette <= 64);
    if from_palette == 0 {
        return 0;
    }
    if from_palette <= 8 {
        return 8;
    }
    from_palette.next_power_of_two()
}

/// The FastIndexBuffer is aiming to optimize for access operations.
/// It accomplishes this by rounding up the index_size to the nearest value that
/// is a power of 2 and by skipping 2 and 4.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct FastIndexBuffer {
    index_size: usize,
    index_size_log_2: usize,
    indices_per_u64: u8,
    mask: u64,
    len: usize,
    storage: Vec<u64>,
}

impl FastIndexBuffer {
    fn set_index_with_index_size(
        &mut self,
        offset: usize,
        index_size: usize,
        index_size_log_2: usize,
        index: usize,
    ) -> usize {
        debug_assert!(index_size > 0);
        #[cfg(all(feature = "unsafe-optimizations", target_endian = "little"))]
        if index_size == 8 {
            let byte_ptr = self.storage.as_mut_ptr() as *mut u8;
            unsafe {
                let slot = byte_ptr.add(offset);
                let old = *slot;
                *slot = index as u8;
                return old as usize;
            }
        }
        let total_bit_offset = offset << index_size_log_2;
        let storage_index = total_bit_offset >> 6;
        let bit_offset = total_bit_offset & 63;
        let mask = (u64::MAX >> (64 - index_size)) << bit_offset;
        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked_mut(storage_index) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &mut self.storage[storage_index]
            }
        };
        let raw = *target_u64;
        let old_index = ((raw & mask) >> bit_offset) as usize;
        let new_raw = (raw & !mask) | ((index as u64) << bit_offset);
        *target_u64 = new_raw;

        old_index
    }

    fn _set_index(&mut self, offset: usize, index: usize) -> usize {
        #[cfg(all(feature = "unsafe-optimizations", target_endian = "little"))]
        if self.index_size == 8 {
            let byte_ptr = self.storage.as_mut_ptr() as *mut u8;
            unsafe {
                let slot = byte_ptr.add(offset);
                let old = *slot;
                *slot = index as u8;
                return old as usize;
            }
        }
        let total_bit_offset = offset << self.index_size_log_2;
        let storage_index = total_bit_offset >> 6;
        let bit_offset = total_bit_offset & 63;
        let mask = self.mask << bit_offset;
        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked_mut(storage_index) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &mut self.storage[storage_index]
            }
        };
        let raw = *target_u64;
        let old_index = ((raw & mask) >> bit_offset) as usize;
        let new_raw = (raw & !mask) | ((index as u64) << bit_offset);
        *target_u64 = new_raw;

        old_index
    }

    fn _get_index(&self, offset: usize) -> usize {
        if self.index_size == 0 {
            return 0;
        }
        #[cfg(all(feature = "unsafe-optimizations", target_endian = "little"))]
        if self.index_size == 8 {
            let byte_ptr = self.storage.as_ptr() as *const u8;
            unsafe {
                let slot = byte_ptr.add(offset);
                return *slot as usize;
            }
        }
        let total_bit_offset = offset << self.index_size_log_2;
        let storage_index = total_bit_offset >> 6;
        let bit_offset = total_bit_offset & 63;
        let mask = self.mask << bit_offset;
        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked(storage_index) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &self.storage[storage_index]
            }
        };
        ((*target_u64 & mask) >> bit_offset) as usize
    }

    /// Writes the indices into the buf, returns amount of indices actually written
    fn get_index_bulk(&self, storage_index: usize, buf: &mut [usize; 8]) -> usize {
        let count = self.indices_per_u64 as usize;

        #[cfg(all(feature = "unsafe-optimizations", target_endian = "little"))]
        if self.index_size == 8 {
            let byte_ptr = self.storage.as_ptr() as *const u8;
            let base_offset = storage_index << self.index_size_log_2;
            unsafe {
                for i in 0..count {
                    buf[i] = *byte_ptr.add(base_offset + i) as usize;
                }
            }
            return self.indices_per_u64 as usize;
        }

        let target_u64 = {
            #[cfg(feature = "unsafe-optimizations")]
            {
                unsafe { self.storage.get_unchecked(storage_index) }
            }
            #[cfg(not(feature = "unsafe-optimizations"))]
            {
                &self.storage[storage_index]
            }
        };

        for i in 0..count {
            let bit_offset = i << self.index_size_log_2;
            buf[i] = ((target_u64 >> bit_offset) & self.mask) as usize;
        }
        count
    }
}

impl IndexBuffer for FastIndexBuffer {
    fn new() -> Self {
        Self {
            index_size: 0,
            index_size_log_2: 0,
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
        self.mask = u64::MAX >> (64 - self.index_size);
        self.storage.resize(needed_u64, 0);
        self.storage.fill(0);
        self.len = len;
    }

    fn clear(&mut self) {
        self.index_size = 0;
        self.index_size_log_2 = 0;
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
        let new_size = map_index_size(new_size);
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
                    self.set_index_with_index_size(
                        i,
                        new_size,
                        new_size.trailing_zeros() as usize,
                        *new_index,
                    );
                }
            } else {
                // No mapping provided, adjust indices without mapping
                // We can work inplace by starting from the end and going backwards
                for i in (0..self.len).rev() {
                    let old_index = self.get_index(i);
                    self.set_index_with_index_size(
                        i,
                        new_size,
                        new_size.trailing_zeros() as usize,
                        old_index,
                    );
                }
            }
            self.indices_per_u64 = new_indices_per_u64 as u8;
            self.mask = u64::MAX >> (64 - new_size);
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
                    self.set_index_with_index_size(
                        i,
                        new_size,
                        new_size.trailing_zeros() as usize,
                        *new_index,
                    );
                }
            } else {
                // No Mapping provided, just truncate indices
                for i in 0..self.len {
                    let index = self.get_index(i);
                    // We can just override the storage in place because the new size is smaller
                    self.set_index_with_index_size(
                        i,
                        new_size,
                        new_size.trailing_zeros() as usize,
                        index,
                    );
                }
            }
            let new_indices_per_u64 = 64 / new_size;
            self.indices_per_u64 = new_indices_per_u64 as u8;
            self.mask = u64::MAX >> (64 - new_size);
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
        self.index_size_log_2 = new_size.trailing_zeros() as usize;
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
        = FastIndexIterator<'a>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        FastIndexIterator {
            buffer: self,
            offset: 0,
            bulk_buf: [0; 8],
            bulk_pos: 0,
            bulk_count: 0,
            storage_index: 0,
        }
    }
}

// ITERATOR
#[derive(Debug, Clone)]
pub struct FastIndexIterator<'a> {
    buffer: &'a FastIndexBuffer,
    offset: usize,
    bulk_buf: [usize; 8],
    bulk_pos: usize,
    bulk_count: usize,
    storage_index: usize,
}

impl<'a> Iterator for FastIndexIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.buffer.len {
            return None;
        }

        if self.buffer.index_size == 0 {
            self.offset += 1;
            return Some(0);
        }

        if self.bulk_pos == self.bulk_count {
            let per_u64 = self.buffer.indices_per_u64 as usize;
            let remaining = self.buffer.len - self.offset;

            self.bulk_count = self
                .buffer
                .get_index_bulk(self.storage_index, &mut self.bulk_buf);

            if remaining < per_u64 {
                self.bulk_count = remaining;
            }

            self.storage_index += 1;
            self.bulk_pos = 0;
        }

        let value = self.bulk_buf[self.bulk_pos];
        self.bulk_pos += 1;
        self.offset += 1;
        Some(value)
    }
}
