use rustc_hash::FxHashMap;
use std::{hash::Hash, ops::Index};

/// A palette compressed vector.
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct PaletteVec<T: Eq + Hash + Clone> {
    palette: Vec<(T, u32)>,
    indices: Vec<u64>,
    /// Amount of bits unused in last u64
    padding_in_last_u64: u8,
    index_size: u8,
    len: usize,
}

impl<T: Eq + Hash + Clone> PaletteVec<T> {
    pub fn new() -> Self {
        Self {
            palette: Vec::new(),
            indices: Vec::new(),
            padding_in_last_u64: 0,
            index_size: 1,
            len: 0,
        }
    }

    pub fn index_iterator(&self) -> IndexIterator {
        IndexIterator {
            index_size: self.index_size,
            indices: &self.indices,
            padding_last_u64: self.padding_in_last_u64,
            current_u64: 0,
            current_offset: 0,
        }
    }

    /// Grows the index size if it's needed to accomodate for a new palette entry.
    fn grow_index_size_if_needed(&mut self) {
        if self.palette.len() >= 1 << self.index_size {
            self.grow_index_size();
        }
    }

    fn grow_index_size(&mut self) {
        let mut new_vec: PaletteVec<T> = PaletteVec::new();
        new_vec.indices = Vec::with_capacity(self.indices.len() * 2);
        new_vec.palette = self.palette.clone();
        new_vec.index_size = self.index_size + 1;
        for index in self.index_iterator() {
            new_vec.push_index(index);
        }
        self.index_size = new_vec.index_size;
        self.indices = new_vec.indices;
        self.padding_in_last_u64 = new_vec.padding_in_last_u64;
        debug_assert!(self.palette == new_vec.palette);
    }

    fn push_index(&mut self, index: u64) {
        debug_assert!(index.leading_zeros() >= 64 - self.index_size as u32);
        if self.padding_in_last_u64 < self.index_size || self.indices.is_empty() {
            self.indices.push(0);
            self.padding_in_last_u64 = 64;
        }
        let len = self.indices.len();
        let last_u64 = &mut self.indices[len - 1];
        *last_u64 |= index << (self.padding_in_last_u64 - self.index_size);
        self.padding_in_last_u64 -= self.index_size;
    }

    /// Pops the last index from the indices vector and adjusts the padding in the last u64.
    /// This does not do any palette manipulation.
    fn pop_index(&mut self) -> Option<u64> {
        if self.indices.is_empty() {
            return None;
        }
        let len = self.indices.len();
        let last_u64 = &mut self.indices[len - 1];

        let shift = self.padding_in_last_u64;
        let mask = (1 << self.index_size) - 1;
        let index = (*last_u64 >> shift) & mask;

        *last_u64 &= !(mask << shift);

        self.padding_in_last_u64 += self.index_size;

        if self.padding_in_last_u64 == 64 {
            self.indices.pop();
            self.padding_in_last_u64 = 64 % self.index_size;
        }
        Some(index)
    }

    fn get_index(&self, index: usize) -> u64 {
        self.get_index_with_index_size(index, self.index_size)
    }

    fn get_index_with_index_size(&self, index: usize, index_size: u8) -> u64 {
        let indices_per_u64 = 64 / index_size as usize;
        let target_u64 = &self.indices[index / indices_per_u64];
        let target_offset = 64 - (index % indices_per_u64 + 1) as u8 * index_size;
        let mask = (1 << index_size) - 1;
        (*target_u64 >> target_offset) & mask
    }

    fn set_index(&mut self, index: usize, value: u64) {
        self.set_index_with_index_size(index, value, self.index_size);
    }

    fn set_index_with_index_size(&mut self, index: usize, value: u64, index_size: u8) {
        let indices_per_u64 = 64 / index_size as usize;
        let target_u64 = &mut self.indices[index / indices_per_u64];
        let target_offset = 64 - (index % indices_per_u64 + 1) as u8 * index_size;
        let mask = (1 << index_size) - 1;
        *target_u64 &= !(mask << target_offset);
        *target_u64 |= value << target_offset;
    }

    pub fn push(&mut self, item: T) {
        // Check if item is in palette
        let mut index = None;
        for (i, (entry, count)) in self.palette.iter_mut().enumerate() {
            if entry == &item {
                index = Some(i);
                *count += 1;
                break;
            }
        }
        if let Some(index) = index {
            // Item is in palette, just push index
            self.push_index(index as u64);
            self.len += 1;
            return;
        }

        // Item is not in palette, create new palette entry
        // - Try replacing an unused palette entry
        for (i, (entry, count)) in self.palette.iter_mut().enumerate() {
            if *count == 0 {
                *count = 1;
                *entry = item;
                self.push_index(i as u64);
                self.len += 1;
                return;
            }
        }
        // - Need completely new entry
        self.palette.push((item, 1));
        self.push_index((self.palette.len() - 1) as u64);
        self.len += 1;
        self.grow_index_size_if_needed();
    }

    pub fn pop(&mut self) -> Option<&T> {
        let Some(index) = self.pop_index() else {
            return None;
        };
        let (item, count) = self.palette.get_mut(index as usize).unwrap();
        *count -= 1;
        self.len -= 1;
        Some(item)
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn set(&mut self, index: usize, item: T) {
        if index >= self.len() {
            panic!("Index out of bounds");
        }

        // Get old index and reduce count
        let indices_per_u64 = 64 / self.index_size as usize;
        let target_u64 = &mut self.indices[index / indices_per_u64];
        let target_offset = 64 - (index % indices_per_u64 + 1) as u8 * self.index_size;
        let mask = (1 << self.index_size) - 1;
        let old_index = (*target_u64 >> target_offset) & mask;
        let (old_item, old_count) = self.palette.get_mut(old_index as usize).unwrap();
        if old_item == &item {
            // Item is the same, do nothing
            return;
        }
        *old_count -= 1;

        // Check if item is in palette
        let mut palette_index = None;
        for (i, (entry, count)) in self.palette.iter_mut().enumerate() {
            if entry == &item {
                palette_index = Some(i);
                *count += 1;
                break;
            }
        }
        if let Some(palette_index) = palette_index {
            // Item is in palette, just push index
            *target_u64 &= !(mask << target_offset);
            *target_u64 |= (palette_index as u64) << target_offset;
            return;
        }

        // Item is not in palette already
        // - Try replacing old entry
        for (i, (entry, count)) in self.palette.iter_mut().enumerate() {
            if *count == 0 {
                *count = 1;
                *entry = item;
                *target_u64 &= !(mask << target_offset);
                *target_u64 |= (i as u64) << target_offset;
                return;
            }
        }
        // - Need completely new entry
        self.palette.push((item, 1));
        let new_index = self.palette.len() - 1;
        *target_u64 &= !(mask << target_offset);
        *target_u64 |= (new_index as u64) << target_offset;
        self.grow_index_size_if_needed();
    }

    pub fn get(&self, index: usize) -> &T {
        if index >= self.len() {
            panic!("Index out of bounds");
        }
        let indices_per_u64 = 64 / self.index_size as usize;
        let target_u64 = &self.indices[index / indices_per_u64];
        let target_offset = 64 - (index % indices_per_u64 + 1) as u8 * self.index_size;
        let mask = (1 << self.index_size) - 1;
        let palette_index = (*target_u64 >> target_offset) & mask;
        let (item, _) = self.palette.get(palette_index as usize).unwrap();
        item
    }
    pub fn get_checked(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        Some(self.get(index))
    }

    pub fn last(&self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        }
        Some(self.get(self.len().saturating_sub(1)))
    }

    pub fn remove(&mut self, index: usize) -> &T {
        if index >= self.len() {
            panic!("Index out of bounds");
        }
        // Get old index and reduce count
        let palette_index = self.get_index(index);
        let (_, count) = self.palette.get_mut(palette_index as usize).unwrap();
        *count -= 1;

        // Move all trailing indices one step to the left
        let len = self.len();
        for i in index..len - 1 {
            let index = self.get_index(i + 1);
            self.set_index(i, index);
        }
        self.pop_index();
        self.len -= 1;
        // Return the removed item
        &self.palette.get(palette_index as usize).unwrap().0
    }

    pub fn swap_remove(&mut self, index: usize) -> &T {
        if index >= self.len() {
            panic!("Index out of bounds");
        }
        if index == self.len() - 1 {
            return self.pop().unwrap();
        }
        // Get old index and reduce count
        let palette_index = self.get_index(index);
        let (_, count) = self.palette.get_mut(palette_index as usize).unwrap();
        *count -= 1;

        // Move last index to the removed index
        let last = self.get_index(self.len() - 1);
        self.set_index(index, last);
        self.pop_index();
        self.len -= 1;

        &self.palette.get(palette_index as usize).unwrap().0
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index > self.len() {
            panic!("Index out of bounds");
        }
        // Move all trailing indices one step to the right
        let len = self.len();
        let last = self.get_index(len - 1);
        self.push_index(last);
        for i in (index..len - 1).rev() {
            let index = self.get_index(i);
            self.set_index(i + 1, index);
        }

        // Does item already exist in palette?
        let mut palette_index = None;
        for (i, (entry, count)) in self.palette.iter_mut().enumerate() {
            if entry == &item {
                palette_index = Some(i);
                *count += 1;
                break;
            }
        }
        if let Some(palette_index) = palette_index {
            self.set_index(index, palette_index as u64);
            self.len += 1;
            return;
        }

        // Can we reuse an existing palette entry?
        for (i, (entry, count)) in self.palette.iter_mut().enumerate() {
            if *count == 0 {
                *count = 1;
                *entry = item;
                self.set_index(index, i as u64);
                self.len += 1;
                return;
            }
        }

        // Need to create a new palette entry
        self.palette.push((item, 1));
        self.set_index(index, (self.palette.len() - 1) as u64);
        self.len += 1;
        self.grow_index_size_if_needed();
    }

    pub fn clear(&mut self) {
        self.palette.clear();
        self.indices.clear();
        self.padding_in_last_u64 = 0;
        self.len = 0;
        self.index_size = 1;
    }

    pub fn clear_keep_index_size(&mut self) {
        self.palette.clear();
        self.indices.clear();
        self.padding_in_last_u64 = 0;
        self.len = 0;
    }

    pub fn get_palette_entry(&self, item: &T) -> Option<(usize, u32)> {
        for (i, (entry, count)) in self.palette.iter().enumerate() {
            if entry == item {
                return Some((i, *count));
            }
        }
        None
    }

    /// DANGER: If you set the palette entry to an item that is already in the palette,
    /// two different indices will now exist for the same item. To circumvent this, you should either:
    /// 
    /// 1) Only set an item to a palette entry that is not already in the palette OR
    /// 2) Call optimize after setting the duplicate palette entry.
    pub fn set_palette_entry(&mut self, palette_index: usize, item: T) {
        if palette_index >= self.palette.len() {
            panic!("Index out of bounds.");
        }
        self.palette[palette_index].0 = item;
    }

    /// Optimizes the palette and indices vector. This is potentially very expensive
    /// and should be done sparingly, but it should be done at some point.
    ///
    /// Most likely you will want to call this: Before serializing the data or
    /// using heuristics like after a specific number of operations have been done
    /// or how much time has passed since last optimization.
    pub fn optimize(&mut self) {
        let mut new_palette = FxHashMap::default();
        for (item, count) in &self.palette {
            if *count > 0 {
                if let Some(count) = new_palette.get_mut(item) {
                    *count += *count;
                } else {
                    new_palette.insert(item.clone(), *count);
                }
            }
        }
        let mut new_palette = new_palette.into_iter().collect::<Vec<_>>();
        new_palette.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        if new_palette.is_empty() {
            // No palette entries left, clear everything
            self.clear();
            return;
        }

        let index_size = ((new_palette.len() as f64).log2().ceil() as u8).max(1);
        assert!(index_size <= self.index_size);

        let mut current_u64 = 0;
        let mut current_indices_index = 0;
        let mut current_padding = 64;
        for i in 0..self.len() {
            let old_index = self.get_index(i);
            let (item, _) = self.palette.get(old_index as usize).unwrap();
            let new_index = new_palette
                .iter()
                .position(|(entry, _)| entry == item)
                .unwrap() as u64;
            if current_padding < index_size {
                self.indices[current_indices_index] = current_u64;
                current_indices_index += 1;
                current_padding = 64;
                current_u64 = 0;
            }
            current_u64 |= new_index << (current_padding - index_size);
            current_padding -= index_size;
        }
        self.indices[current_indices_index] = current_u64;
        if current_padding < 64 {
            current_indices_index += 1;
        }
        self.indices.truncate(current_indices_index);

        self.palette = new_palette;
        self.index_size = index_size;
        self.padding_in_last_u64 = current_padding;
    }

    pub fn map_palette<M: Eq + Hash + Clone>(&self, mut f: impl FnMut(&T) -> M) -> PaletteVec<M>{
        PaletteVec{
            palette: self.palette.iter().map(|(element, count)|(f(element), *count)).collect(),
            indices: self.indices.clone(),
            len: self.len,
            index_size: self.index_size,
            padding_in_last_u64: self.padding_in_last_u64,
        }
    }
}

pub struct IndexIterator<'a> {
    index_size: u8,
    indices: &'a [u64],
    padding_last_u64: u8,
    current_u64: usize,
    current_offset: u8,
}

#[rustfmt::skip]
impl<'a> Iterator for IndexIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.padding_last_u64 <= 64 - self.index_size);
        if self.indices.is_empty() {
            return None;
        }

        if self.current_u64 == self.indices.len() - 1 && self.current_offset == 64 - self.padding_last_u64 {
            return None;
        }
        if 64 - self.current_offset < self.index_size {
            self.current_u64 += 1;
            self.current_offset = 0;
        }
        let indices_u64 = self.indices.get(self.current_u64)?;
        let item = Some((indices_u64 >> (64 - self.current_offset - self.index_size)) & ((1 << self.index_size) - 1));
        self.current_offset += self.index_size;
        item
    }
}

impl<T: Eq + Hash + Clone> Index<usize> for PaletteVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

pub struct PaletteVecIterator<'a, T: Eq + Hash + Clone> {
    vec: &'a PaletteVec<T>,
    index: usize,
}

impl<'a, T: Eq + Hash + Clone> Iterator for PaletteVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            return None;
        }
        self.index += 1;
        Some(self.vec.get(self.index - 1))
    }
}

/// This clones every item from the palette. This may be expensive.
pub struct PaletteVecIteratorOwned<T: Eq + Hash + Clone> {
    vec: PaletteVec<T>,
    index: usize,
}

impl<T: Eq + Hash + Clone> Iterator for PaletteVecIteratorOwned<T> {
    type Item = T;

    /// This clones every item from the palette. This may be expensive.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            return None;
        }
        self.index += 1;
        Some(self.vec.get(self.index - 1).clone())
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn test_pushing_doesnt_panic() {
        let mut vec = PaletteVec::new();
        for i in 0..200 {
            for j in 0..i {
                vec.push(j);
            }
        }
    }

    #[test]
    fn test_popping_empty() {
        let mut vec = PaletteVec::<u32>::new();
        for _ in 0..200 {
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_pushing_and_popping() {
        let mut vec = PaletteVec::new();
        for i in 0..1000 {
            vec.push(i);
            assert_eq!(vec.pop(), Some(&i));
        }
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_pushing_and_popping2() {
        let mut vec = PaletteVec::new();
        for i in 0..500 {
            for j in 0..i {
                vec.push(j);
            }
            for j in (0..i).rev() {
                assert_eq!(vec.pop(), Some(&j));
            }
        }
        assert_eq!(vec.pop(), None);

        for i in 0..500 {
            for j in 0..i {
                vec.push(j);
            }
            for j in (0..i).rev() {
                assert_eq!(vec.pop(), Some(&j));
            }
        }
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_push_pop_random() {
        let mut vec = PaletteVec::<u32>::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..30 {
            for _ in 0..1000 {
                let i = rand::random::<u32>() % 333;
                vec.push(i);
                control.push(i);
            }
            for _ in 0..1000 {
                let i = control.pop().unwrap();
                assert_eq!(vec.pop(), Some(&i));
            }
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_set() {
        let mut vec = PaletteVec::new();
        for i in 0..1000 {
            vec.push(i);
        }
        for i in 0..1000 {
            vec.set(i, i + 1000);
        }
        for i in (0..1000).rev() {
            assert_eq!(vec.pop(), Some(&(i + 1000)));
        }
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_set_random() {
        let mut vec = PaletteVec::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..30 {
            for _ in 0..1000 {
                let i = rand::random::<u32>() % 333;
                vec.push(i);
                control.push(i);
            }
            for _ in 0..1000 {
                let i = rand::random::<u32>() % 333;
                let j = rand::random::<u32>() % 333;
                vec.set(i as usize, j);
                control[i as usize] = j;
            }
            for _ in 0..1000 {
                let i = control.pop().unwrap();
                assert_eq!(vec.pop(), Some(&i));
            }
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_get() {
        let mut vec = PaletteVec::new();
        for _ in 0..33 {
            for i in 0..1000 {
                vec.push(i);
            }
            for i in 0..1000 {
                assert_eq!(vec.get(i), &i);
            }
            for i in (0..1000).rev() {
                assert_eq!(vec.pop(), Some(&i));
            }
        }
    }

    #[test]
    fn test_last() {
        let mut vec = PaletteVec::new();
        for _ in 0..33 {
            for i in 0..1000 {
                vec.push(i);
                assert_eq!(vec.last(), Some(&i));
            }
            for i in (0..1000).rev() {
                assert_eq!(vec.pop(), Some(&i));
            }
        }
    }

    #[test]
    fn test_remove() {
        let mut vec = PaletteVec::new();
        for _ in 0..33 {
            for i in 0..1000 {
                vec.push(i);
            }
            for i in 0..1000 {
                assert_eq!(vec.remove(0), &i);
            }
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_remove_random() {
        let mut rng = rand::thread_rng();
        let mut vec = PaletteVec::<u32>::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..33 {
            for _ in 0..500 {
                let n = rng.gen_range(0..333);
                vec.push(n);
                control.push(n);
            }
            for _ in 0..300 {
                let i = rng.gen_range(0..control.len());
                vec.remove(i);
                control.remove(i);
            }
            for i in 0..control.len() {
                assert_eq!(vec.get(i), &control[i]);
            }
        }
    }

    #[test]
    fn test_swap_remove() {
        let mut vec = PaletteVec::<u32>::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..11 {
            for i in 0..1000 {
                vec.push(i);
                control.push(i);
            }
            for _ in 0..1000 {
                assert_eq!(*vec.swap_remove(0), control.swap_remove(0));
            }
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_swap_remove_random() {
        let mut rng = rand::thread_rng();
        let mut vec = PaletteVec::<u32>::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..33 {
            for _ in 0..500 {
                let n = rng.gen_range(0..333);
                vec.push(n);
                control.push(n);
            }
            for _ in 0..333 {
                let i = rng.gen_range(0..control.len());
                assert_eq!(*vec.swap_remove(i), control.swap_remove(i));
            }
            for i in 0..control.len() {
                assert_eq!(vec.get(i), &control[i]);
            }
        }
    }

    #[test]
    fn test_insert() {
        let mut vec = PaletteVec::new();
        for i in 0..1000 {
            vec.push(i);
        }
        for i in 0..1000 {
            vec.insert(i, i + 1000);
        }
        for i in (0..1000).rev() {
            assert_eq!(vec.get(i), &(i + 1000));
        }
    }

    #[test]
    fn test_insert_random() {
        let mut vec = PaletteVec::<u32>::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..33 {
            for _ in 0..500 {
                let i = rand::random::<u32>() % 333;
                vec.push(i);
                control.push(i);
            }
            for _ in 0..500 {
                let i = rand::random::<u32>() % 333;
                let j = rand::random::<u32>() % 333;
                vec.insert(i as usize, j);
                control.insert(i as usize, j);
            }
            for i in 0..500 {
                assert_eq!(vec.get(i), &control[i]);
            }
        }
    }

    #[test]
    fn test_clear() {
        let mut vec = PaletteVec::new();
        for _ in 0..33 {
            for i in 0..1000 {
                vec.push(i);
            }
            vec.clear();
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_clear_keep_index_size() {
        let mut vec = PaletteVec::new();
        for _ in 0..33 {
            for i in 0..1000 {
                vec.push(i);
            }
            vec.clear_keep_index_size();
            assert_eq!(vec.pop(), None);
        }
    }

    #[test]
    fn test_optimize() {
        let mut rng = thread_rng();
        let mut vec = PaletteVec::new();
        let mut control = Vec::<u32>::new();
        for _ in 0..203 {
            for i in 0..rng.gen_range(100..1000) {
                vec.push(i);
                control.push(i);
            }
            for _ in 0..rng.gen_range(10..90) {
                let i = rng.gen_range(0..control.len());
                vec.remove(i);
                control.remove(i);
            }
            for _ in 0..rng.gen_range(10..90) {
                let i = rng.gen_range(0..control.len());
                let n = rng.gen_range(0..333);
                vec.insert(i, n);
                control.insert(i, n);
            }
            vec.optimize();
        }
        assert_eq!(vec.len(), control.len());
        for i in 0..control.len() {
            assert_eq!(vec.get(i), &control[i]);
        }
    }

    #[test]
    fn test_large() {
        let mut rng = thread_rng();
        let mut vec = PaletteVec::<u32>::new();
        vec.optimize();
        let mut control = Vec::<u32>::new();
        for _ in 0..533 {
            // Push 100 random numbers
            for _ in 0..1000 {
                let n = rng.gen_range(0..514);
                vec.push(n);
                control.push(n);
            }
            // Remove first number
            vec.remove(0);
            control.remove(0);
            // Remove 100 random numbers
            for _ in 0..100 {
                let i = rng.gen_range(0..control.len());
                vec.remove(i as usize);
                control.remove(i as usize);
            }
            // Insert 100 random numbers
            for _ in 0..100 {
                let i = rng.gen_range(0..control.len());
                let n = rng.gen_range(0..514);
                vec.insert(i as usize, n);
                control.insert(i as usize, n);
            }
            // Pop random numbers
            for _ in 0..100 {
                assert_eq!(vec.pop(), control.pop().as_ref());
            }
            // Swap remove random numbers
            for _ in 0..100 {
                let i = rng.gen_range(0..control.len());
                assert_eq!(
                    vec.swap_remove(i as usize),
                    &control.swap_remove(i as usize)
                );
            }
            // Optimize randomly
            if rng.gen_bool(0.25) {
                vec.optimize();
            }
            assert!(vec.len() == control.len());
        }
        vec.optimize();
        assert_eq!(vec.len(), control.len());
        while let Some(i) = vec.pop() {
            assert_eq!(i, &control.pop().unwrap());
        }
        vec.optimize();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.index_size, 1);
    }

    #[test]
    fn test_optimize_size() {
        let mut vec = PaletteVec::new();
        for i in 0..1000 {
            vec.push(i);
        }
        for _ in 0..999 {
            vec.pop();
        }
        vec.optimize();
        assert_eq!(vec.index_size, 1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.palette.len(), 1);
        assert_eq!(vec.indices.len(), 1);
    }

    #[test]
    fn test_replacing_palette_entry() {
        let mut vec = PaletteVec::new();
        for i in 0..1000 {
            vec.push(i % 2);
        }
        let (index, _) = vec.get_palette_entry(&0).unwrap();
        vec.set_palette_entry(index, 1);
        vec.optimize();
        while let Some(i) = vec.pop() {
            assert_eq!(i, &1);
        }
    }
}
