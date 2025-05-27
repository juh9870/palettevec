use std::collections::HashMap;

pub mod hybrid;

pub struct PaletteEntry<T: Eq + Clone> {
    value: T,
    count: u32,
}

pub trait Palette<T: Eq> {
    fn get_by_value(&self, value: &T) -> Option<&PaletteEntry<T>>;
    fn get_mut_by_value(&mut self, value: &T) -> Option<&mut PaletteEntry<T>>;
    fn get_by_index(&self, index: u64) -> Option<&PaletteEntry<T>>;
    fn get_mut_by_index(&mut self, index: u64) -> Option<&mut PaletteEntry<T>>;

    /// Assumes that the palette doesn't contain this value yet.
    /// Returns the new index.
    /// This function is not allowed to change any of the other indices.
    fn insert_new(&mut self, entry: PaletteEntry<T>) -> u64;
    /// Optimizes the palette and returns the new mapping of index -> value
    /// if necessary.
    fn optimize(&mut self) -> Option<HashMap<u64, T>>;
}
