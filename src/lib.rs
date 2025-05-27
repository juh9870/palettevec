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

pub struct SpecializedPaletteArray<T> {
    inner: Vec<T>,
}

impl<T: Clone> SpecializedPaletteArray<T> {
    pub fn new(size: usize, initial_value: T) -> Self {
        Self {
            inner: vec![initial_value; size],
        }
    }
}

impl<T: Clone> PaletteArray<T> for SpecializedPaletteArray<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, index: usize) -> &T {
        &self.inner[index]
    }

    fn set(&mut self, index: usize, value: &T) {
        self.inner[index] = value.clone();
    }
}
