use std::collections::HashMap;

pub(crate) mod aligned;

pub trait IndexBuffer {
    fn new() -> Self;
    /// Clears itself and fills itself with len 0-indices.
    ///
    /// When index_size = 0 this should just set the len.
    fn zeroed(&mut self, len: usize);

    /// Returns the number of indices in the buffer.
    fn len(&self) -> u64;
    fn is_empty(&self) -> bool;
    /// Call this every time the index buffer is resized.
    /// New mapping is <old_index, new_index>
    ///
    /// ALLOWED INDEX SIZES: [0, 63]
    fn set_index_size(&mut self, new_size: usize, new_mapping: Option<HashMap<u64, u64>>);

    /// index_offset in indices, not bits
    /// returns the old index
    fn set_index(&mut self, index_offset: usize, index: u64) -> u64;
    /// index_offset in indices, not bits
    ///
    /// Out of bounds access is checked in palettevec
    fn get_index(&self, index_offset: usize) -> u64;

    fn push_index(&mut self, index: u64);
    fn pop_index(&mut self) -> Option<u64>;
}
