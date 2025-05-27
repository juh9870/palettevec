use crate::PaletteArray;

pub struct FastPaletteArray<T> {
    inner: Vec<T>,
}

impl<T: Clone> FastPaletteArray<T> {
    pub fn new(size: usize, initial_value: T) -> Self {
        Self {
            inner: vec![initial_value; size],
        }
    }
}

impl<T: Clone> PaletteArray<T> for FastPaletteArray<T> {
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
