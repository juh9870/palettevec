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
            let (index, new_index_size) = self.palette.insert_new(PaletteEntry {
                value: value,
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
}
