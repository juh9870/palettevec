use std::collections::HashMap;

pub(crate) mod aligned;

pub trait IndexBuffer {
    /// Call this every time the index buffer is resized.
    /// New mapping is <old_index, new_index>
    fn set_index_size(&mut self, new_size: usize, new_mapping: Option<HashMap<u64, u64>>);

    /// index_offset in indices, not bits
    fn set_index(&mut self, index_offset: usize, index: u64);
    /// index_offset in indices, not bits
    fn get_index(&self, index_offset: usize) -> u64;

    fn push_index(&mut self, index: u64);
    fn pop_index(&mut self) -> Option<u64>;
}
