use std::{hash::Hash, marker::PhantomData};

use index_buffer::IndexBuffer;
use palette::{Palette, PaletteEntry};

pub(crate) mod index_buffer;
pub(crate) mod palette;

#[cfg(test)]
pub(crate) mod tests;

pub struct PaletteVec<T: Eq + Hash + Clone, P: Palette<T>, B: IndexBuffer> {
    palette: P,
    buffer: B,
    phantom: PhantomData<T>,
}

impl<T: Eq + Hash + Clone, P: Palette<T>, B: IndexBuffer> PaletteVec<T, P, B> {
    pub fn new() -> Self {
        Self {
            palette: P::new(),
            buffer: B::new(),
            phantom: PhantomData,
        }
    }

    pub fn filled(value: T, len: usize) -> Self {
        let mut palette = P::new();
        let (index, index_size) = palette.insert_new(PaletteEntry { value, count: len });
        debug_assert_eq!(index, 0);
        let mut buffer = B::new();
        if let Some(index_size) = index_size {
            buffer.set_index_size(index_size, None);
        }
        buffer.zeroed(len);

        Self {
            palette,
            buffer,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn unique_values(&self) -> usize {
        self.palette.len()
    }

    pub fn push_ref(&mut self, value: &T) {
        let Some((entry, index)) = self.palette.get_mut_by_value(value) else {
            // Value is new, insert it into the palette
            let (index, new_index_size) = self.palette.insert_new(PaletteEntry {
                value: value.clone(),
                count: 1,
            });
            if let Some(new_index_size) = new_index_size {
                self.buffer.set_index_size(new_index_size, None);
            }
            self.buffer.push_index(index);
            return;
        };
        // Value is already in the palette, increment its count
        entry.count += 1;
        self.buffer.push_index(index);
    }

    /// Prefer `push_ref` when possible, as it avoids cloning the value if the value is already in the palette.
    pub fn push(&mut self, value: T) {
        let Some((entry, index)) = self.palette.get_mut_by_value(&value) else {
            // Value is new, insert it into the palette
            let (index, new_index_size) = self.palette.insert_new(PaletteEntry { value, count: 1 });
            if let Some(new_index_size) = new_index_size {
                self.buffer.set_index_size(new_index_size, None);
            }
            self.buffer.push_index(index);
            return;
        };
        // Value is already in the palette, increment its count
        entry.count += 1;
        self.buffer.push_index(index);
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.buffer.pop_index()?;
        let entry = self.palette.get_mut_by_index(index)?;
        entry.count -= 1;
        let value = entry.value.clone();
        if entry.count == 0 {
            self.palette.mark_as_unused(index);
        }
        Some(value)
    }

    pub fn set(&mut self, offset: usize, value: &T) {
        let old_index_size = self.palette.index_size();
        // Check if the value is already in the palette
        if let Some((entry, index)) = self.palette.get_mut_by_value(value) {
            if old_index_size == 0 {
                // Reaching this means we have an index size of 0 and set was called
                // with an element equal to the only element that exists in the palette vec.
                // So we can just return;
                return;
            }
            let old_index = self.buffer.set_index(offset, index);
            if old_index != index {
                entry.count += 1;
                let old_entry = self.palette.get_mut_by_index(old_index).unwrap();
                old_entry.count -= 1;
                if old_entry.count == 0 {
                    self.palette.mark_as_unused(old_index);
                }
            }
            return;
        }

        // Value is new, insert into palette
        let (new_index, new_index_size) = self.palette.insert_new(PaletteEntry {
            value: value.clone(),
            count: 1,
        });
        if let Some(new_index_size) = new_index_size {
            self.buffer.set_index_size(new_index_size, None);
        }
        let old_index = self.buffer.set_index(offset, new_index);
        let old_entry = self.palette.get_mut_by_index(old_index).unwrap();
        old_entry.count -= 1;
        if old_entry.count == 0 {
            self.palette.mark_as_unused(old_index);
        }
    }

    pub fn get(&self, offset: usize) -> Option<T> {
        if offset >= self.buffer.len() {
            return None;
        }
        let index = self.buffer.get_index(offset);
        Some(self.palette.get_by_index(index).unwrap().value.clone())
    }

    /// Optimizes the palette and indices vector. This is potentially very expensive
    /// and should be done sparingly, but it should be done at some point.
    ///
    /// Most likely you will want to call this: Before serializing the data or
    /// using heuristics like after a specific number of set/push operations have been done,
    /// how large the unique_values() difference is compared to earlier
    /// or how much time has passed since last optimization.
    pub fn optimize(&mut self) {
        let mapping = self.palette.optimize();
        let new_index_size = self.palette.index_size();
        self.buffer.set_index_size(new_index_size, mapping);
    }
}

impl<T: Eq + Hash + Clone, P: Palette<T>, B: IndexBuffer> Default for PaletteVec<T, P, B> {
    fn default() -> Self {
        Self::new()
    }
}
