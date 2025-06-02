//! # PaletteVec
//!
//! `PaletteVec` is a space-efficient data structure for storing collections
//! with a limited set of repeated elements. It uses a palette-based encoding
//! scheme, similar to how indexed color images or Minecraft chunk data are stored.
//!
//! This approach can lead to significant memory savings when dealing with
//! repetitive data, while still allowing for direct manipulation of the collection.
//!
//! ## Core Components
//!
//! - **`PaletteVec<T, P, B>`:** The main data structure.
//!   - `T`: The type of elements stored. Must be `Eq + Hash + Clone`.
//!   - `P`: The `Palette` implementation (e.g., `HybridPalette`).
//!   - `B`: The `IndexBuffer` implementation (e.g., `AlignedIndexBuffer`).
//! - **`Palette<T>` trait:** Defines the interface for palette implementations.
//! - **`IndexBuffer` trait:** Defines the interface for how indices are stored.
use std::{hash::Hash, marker::PhantomData, ops::Add};

use index_buffer::IndexBuffer;
use palette::{Palette, PaletteEntry};

use crate::palette::CountType;

pub mod index_buffer;
pub mod palette;

#[cfg(test)]
pub(crate) mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct MemoryUsage {
    pub stack: usize,
    pub heap_actually_needed: usize,
    pub heap_allocated: usize,
}

impl Add for MemoryUsage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            stack: self.stack + other.stack,
            heap_actually_needed: self.heap_actually_needed + other.heap_actually_needed,
            heap_allocated: self.heap_allocated + other.heap_allocated,
        }
    }
}

/// A vector-like data structure that uses a palette to store unique elements,
/// significantly reducing memory for collections with many repeated values.
///
/// `T`: The type of elements stored. Must implement `Eq`, `Hash`, and `Clone`. \
/// `P`: The `Palette` implementation used to manage unique elements. \
/// `B`: The `IndexBuffer` implementation used to store indices into the palette.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
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
        let (index, index_size) = palette.insert_new(PaletteEntry {
            value,
            count: len as CountType,
        });
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

    /// Quickly estimates the memory used by the PaletteVec.
    ///
    /// IMPORTANT: Because of technical reasons, this is just an estimate,
    /// not an exact value. Still, it is precise enough to work with.
    pub fn memory_usage(&self) -> MemoryUsage {
        self.palette.memory_usage() + self.buffer.memory_usage()
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

    pub fn get(&self, offset: usize) -> Option<&T> {
        if offset >= self.buffer.len() {
            return None;
        }
        let index = self.buffer.get_index(offset);
        Some(&self.palette.get_by_index(index).unwrap().value)
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

    pub fn iter(&self) -> PaletteVecIter<T, P, B> {
        self.into_iter()
    }

    pub fn iter_palette_entries(&self) -> P::EntriesIter<'_> {
        self.palette.iter()
    }

    /// Returns a mutable iterator over the palette entries.
    /// Each item is a `&mut PaletteEntry<T>`, allowing modification of the value and its count.
    ///
    /// IMPORTANT: Do NOT change the count and make sure you do not introduce duplicate
    /// palette entries (duplicate values).
    pub fn iter_palette_entries_mut(&mut self) -> P::EntriesIterMut<'_> {
        self.palette.iter_mut()
    }
}

impl<T: Eq + Hash + Clone, P: Palette<T>, B: IndexBuffer> Default for PaletteVec<T, P, B> {
    fn default() -> Self {
        Self::new()
    }
}

// ITERATOR
pub struct PaletteVecIter<'a, T, P, B>
where
    T: Eq + Hash + Clone,
    P: Palette<T> + 'a,
    B: IndexBuffer + 'a,
{
    palette: &'a P,
    buffer_iter: B::Iter<'a>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, P, B> Iterator for PaletteVecIter<'a, T, P, B>
where
    T: Eq + Hash + Clone,
    P: Palette<T> + 'a,
    B: IndexBuffer + 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.buffer_iter.next()?;
        let entry = self.palette.get_by_index(idx).unwrap();
        Some(&entry.value)
    }
}

impl<'a, T, P, B> IntoIterator for &'a PaletteVec<T, P, B>
where
    T: Eq + Hash + Clone + 'a,
    P: Palette<T> + 'a,
    B: IndexBuffer + 'a,
{
    type Item = &'a T;
    type IntoIter = PaletteVecIter<'a, T, P, B>;

    fn into_iter(self) -> Self::IntoIter {
        PaletteVecIter {
            palette: &self.palette,
            buffer_iter: self.buffer.iter(),
            phantom: PhantomData,
        }
    }
}
