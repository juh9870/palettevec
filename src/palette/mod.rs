//! The Palette dictates how the actual values you insert into a PaletteVec are stored and accessed.
//!
//! HybridPalette is a good default.

use std::cmp::Ordering;

use rustc_hash::FxHashMap;

use crate::MemoryUsage;

pub mod hybrid;
pub mod vec;

pub use self::hybrid::HybridPalette;

// Highest priority: usize
#[cfg(feature = "count-usize")]
pub type CountType = usize;

#[cfg(all(not(feature = "count-usize"), feature = "count-u64"))]
pub type CountType = u64;

#[cfg(all(
    not(feature = "count-usize"),
    not(feature = "count-u64"),
    feature = "count-u32"
))]
pub type CountType = u32;

#[cfg(all(
    not(feature = "count-usize"),
    not(feature = "count-u64"),
    not(feature = "count-u32"),
    feature = "count-u16"
))]
pub type CountType = u16;

// Demand usage of a feature
#[cfg(all(
    not(feature = "count-usize"),
    not(feature = "count-u64"),
    not(feature = "count-u32"),
    not(feature = "count-u16"),
))]
pub type CountType = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct PaletteEntry<T: Eq + Clone> {
    pub value: T,
    pub count: CountType,
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

pub(crate) fn calculate_smallest_index_size(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    (usize::BITS - (n - 1).leading_zeros()) as usize
}

pub trait Palette<T: Eq + Clone>: Clone {
    fn new() -> Self;
    /// Returns amount of palette entries with count > 0.
    /// DO NOT use this to calculate index size. Use index_size() instead.
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn memory_usage(&self) -> MemoryUsage;
    /// Gets the current index size. This can change after insert_new() or optimize().
    fn index_size(&self) -> usize;

    //fn get_by_value(&self, value: &T) -> Option<(&PaletteEntry<T>, usize)>;
    fn get_mut_by_value(&mut self, value: &T) -> Option<(&mut PaletteEntry<T>, usize)>;
    fn get_by_index(&self, index: usize) -> Option<&PaletteEntry<T>>;
    fn get_mut_by_index(&mut self, index: usize) -> Option<&mut PaletteEntry<T>>;

    /// IMPORTANT: Call this immediately after setting a palette entries count to 0.
    fn mark_as_unused(&mut self, index: usize);

    /// Assumes that the palette doesn't contain this value yet.
    /// Returns the new index and the new index size if needed.
    /// This function is not allowed to change any of the other indices.
    fn insert_new(&mut self, entry: PaletteEntry<T>) -> (usize, Option<usize>);
    /// Optimizes the palette and returns the mapping of old_index -> new_index
    /// if necessary.
    fn optimize(&mut self) -> Option<FxHashMap<usize, usize>>;

    // REF ITERATOR
    type EntriesIter<'a>: Iterator<Item = &'a PaletteEntry<T>>
    where
        Self: 'a,
        T: 'a;

    /// Returns an iterator over the palette entries.
    fn iter(&self) -> Self::EntriesIter<'_>;

    // MUT ITERATOR
    type EntriesIterMut<'a>: Iterator<Item = &'a mut PaletteEntry<T>>
    where
        Self: 'a,
        T: 'a;

    /// Returns a mutable iterator over the palette entries.
    /// Allows modifying entries in place. If an entry's count is set to 0,
    /// 'mark_as_unused' is NOT automatically called for that entry by this iterator.
    /// The caller should ensure palette invariants are maintained, possibly by calling 'optimize()' later.
    fn iter_mut(&mut self) -> Self::EntriesIterMut<'_>;
}
