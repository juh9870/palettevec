pub(crate) mod index_buffer;
pub(crate) mod palette;

#[cfg(test)]
pub(crate) mod tests;

pub trait PaletteArray<T: Clone> {
    fn len(&self) -> usize;

    /// Panics if the index is out of bounds.
    fn get(&self, index: usize) -> &T;

    /// Panics if the index is out of bounds.
    /// This may clone the value if needed.
    fn set(&mut self, index: usize, value: &T);
}
