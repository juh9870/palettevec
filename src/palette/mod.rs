use std::{cmp::Ordering, collections::HashMap};

pub mod hybrid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteEntry<T: Eq + Clone> {
    pub value: T,
    pub count: u32,
}

/// Some with max count will be first, None will be last
fn compare_palette_entries_option_max_first<T: Eq + Clone>(
    a: &Option<PaletteEntry<T>>,
    b: &Option<PaletteEntry<T>>,
) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => b.count.cmp(&a.count),
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
    }
}

/// Max count will be first
fn compare_palette_entries_max_first<T: Eq + Clone>(
    a: &PaletteEntry<T>,
    b: &PaletteEntry<T>,
) -> Ordering {
    b.count.cmp(&a.count)
}

pub trait Palette<T: Eq + Clone> {
    /// Returns amount of palette entries with count > 0.
    /// DO NOT use this to calculate index size. Use index_size() instead.
    fn len(&self) -> usize;
    /// Gets the current index size. This can change after insert_new() or optimize().
    fn index_size(&self) -> usize;

    fn get_by_value(&self, value: &T) -> Option<&PaletteEntry<T>>;
    fn get_mut_by_value(&mut self, value: &T) -> Option<&mut PaletteEntry<T>>;
    fn get_by_index(&self, index: u64) -> Option<&PaletteEntry<T>>;
    fn get_mut_by_index(&mut self, index: u64) -> Option<&mut PaletteEntry<T>>;

    fn get_index_from_value(&self, value: &T) -> Option<u64>;

    /// IMPORTANT: Call this immediately after setting a palette entries count to 0.
    fn mark_as_unused(&mut self, index: u64);

    /// Assumes that the palette doesn't contain this value yet.
    /// Returns the new index.
    /// This function is not allowed to change any of the other indices.
    fn insert_new(&mut self, entry: PaletteEntry<T>) -> u64;
    /// Optimizes the palette and returns the mapping of old_index -> new_index
    /// if necessary.
    fn optimize(&mut self) -> Option<HashMap<u64, u64>>;
}
