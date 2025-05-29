use crate::{index_buffer::IndexBuffer, palette::Palette, PaletteVec};

mod base;

fn test_palette_vec_new<P, B>()
where
    P: Palette<i32>,
    B: IndexBuffer,
{
    for _ in 0..100 {
        let _: PaletteVec<i32, P, B> = PaletteVec::new();
    }
}

fn test_palette_vec_push_pop<P, B>(iteration_count: u32)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);

    // Push
    for _ in 0..iteration_count {
        pv.push(1);
    }
    for _ in (0..iteration_count).rev() {
        assert_eq!(pv.pop(), Some(1));
    }
    assert_eq!(pv.pop(), None);
    for i in 0..iteration_count {
        pv.push(i % 69);
    }
    for i in (0..iteration_count).rev() {
        assert_eq!(pv.pop(), Some(i % 69));
    }
    assert_eq!(pv.pop(), None);
}

fn test_palette_vec_push_ref_pop<P, B>(iteration_count: u32)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);

    // Push_ref
    for _ in 0..iteration_count {
        pv.push_ref(&1);
    }
    for _ in (0..iteration_count).rev() {
        assert_eq!(pv.pop(), Some(1));
    }
    assert_eq!(pv.pop(), None);
    for i in 0..iteration_count {
        pv.push_ref(&(i % 69));
    }
    for i in (0..iteration_count).rev() {
        assert_eq!(pv.pop(), Some(i % 69));
    }
    assert_eq!(pv.pop(), None);
}

fn test_palette_vec_len<P, B>(iteration_count: usize)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);
    assert!(pv.is_empty());
    assert_eq!(pv.len(), 0);
    for i in 1..iteration_count + 1 {
        pv.push(0);
        assert_eq!(pv.len(), i);
        assert!(!pv.is_empty());
    }
    for i in (0..iteration_count).rev() {
        pv.pop();
        assert_eq!(pv.len(), i);
    }
    assert_eq!(pv.len(), 0);
    assert!(pv.is_empty());
}

fn test_palette_vec_unique_values<P, B>(amount_unique_values: usize, iteration_count: usize)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    assert!(
        amount_unique_values < iteration_count,
        "amount_unique_values must be less than iteration_count"
    );
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);
    assert_eq!(pv.len(), 0);
    assert_eq!(pv.unique_values(), 0);
    for i in 0..iteration_count {
        let value = i % amount_unique_values;
        pv.push(value as u32);
        if i < amount_unique_values {
            assert_eq!(pv.unique_values(), i + 1);
        }
    }
    assert_eq!(pv.unique_values(), amount_unique_values);
    for _ in 0..iteration_count {
        pv.pop();
    }
}

fn test_palette_vec_set<P, B>(amount_unique_values: usize, iteration_count: usize)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    assert!(
        amount_unique_values < iteration_count,
        "amount_unique_values must be less than iteration_count"
    );
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);
    assert_eq!(pv.len(), 0);
    assert_eq!(pv.unique_values(), 0);
    for i in 0..iteration_count {
        let value = i % amount_unique_values;
        pv.push(value as u32);
    }
    for i in 0..iteration_count {
        let value = (i + 1) % amount_unique_values;
        pv.set(i as usize, &(value as u32));
    }
    for i in (0..iteration_count).rev() {
        let value = (i + 1) % amount_unique_values;
        assert_eq!(pv.pop(), Some(value as u32));
    }
    assert_eq!(pv.pop(), None);
    assert_eq!(pv.len(), 0);
    assert_eq!(pv.unique_values(), 0);
    for i in 0..iteration_count {
        let value = i % amount_unique_values;
        pv.push(value as u32);
    }
    for i in 0..iteration_count {
        pv.set(i as usize, &0);
    }
    for _ in 0..iteration_count {
        assert_eq!(pv.pop(), Some(0));
    }
    assert_eq!(pv.pop(), None);
    assert_eq!(pv.len(), 0);
    assert_eq!(pv.unique_values(), 0);

    // Set with unique values
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    for _ in 0..iteration_count {
        pv.push(0);
    }
    for i in 0..iteration_count {
        let value = i % amount_unique_values;
        pv.set(i as usize, &(value as u32));
        assert_eq!(pv.get(i), Some(value as u32));
    }
    for i in (0..iteration_count).rev() {
        let value = i % amount_unique_values;
        assert_eq!(pv.pop(), Some(value as u32));
    }
    for i in 0..iteration_count {
        assert_eq!(pv.get(i), None);
    }
    assert_eq!(pv.pop(), None);
    assert_eq!(pv.len(), 0);
    assert_eq!(pv.unique_values(), 0);
}

fn test_palette_vec_get<P, B>(amount_unique_values: usize, iteration_count: usize)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);
    assert_eq!(pv.len(), 0);
    assert_eq!(pv.unique_values(), 0);
    for i in 0..iteration_count {
        let value = i % amount_unique_values;
        pv.push(value as u32);
        assert_eq!(pv.get(i), Some(value as u32));
    }
    for i in 0..iteration_count {
        let value = i % amount_unique_values;
        assert_eq!(pv.get(i), Some(value as u32));
    }
}

fn test_palette_vec_filled<P, B>(iteration_count: usize)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::filled(7, iteration_count);
    for i in 0..iteration_count {
        assert_eq!(pv.get(i), Some(7));
    }
    assert_eq!(pv.get(iteration_count), None);
    for i in 0..iteration_count {
        let value = i % 11;
        pv.set(i as usize, &(value as u32));
        assert_eq!(pv.get(i), Some(value as u32));
    }
}

fn test_palette_vec_optimize<P, B>(iteration_count: usize)
where
    P: Palette<u32>,
    B: IndexBuffer,
{
    assert!(
        iteration_count > 77,
        "For this test, iteration_count needs to be > 77"
    );
    let mut pv: PaletteVec<u32, P, B> = PaletteVec::filled(7, iteration_count);
    assert_eq!(pv.unique_values(), 1);
    assert_eq!(pv.len(), iteration_count);
    pv.optimize();
    assert_eq!(pv.unique_values(), 1);
    assert_eq!(pv.len(), iteration_count);
    pv.set(0, &6);
    assert_eq!(pv.unique_values(), 2);
    assert_eq!(pv.len(), iteration_count);
    pv.optimize();
    assert_eq!(pv.unique_values(), 2);
    assert_eq!(pv.len(), iteration_count);
    for i in 0..iteration_count {
        pv.set(i as usize, &(i as u32 % 77));
    }
    assert_eq!(pv.unique_values(), 77);
    assert_eq!(pv.len(), iteration_count);
    pv.optimize();
    assert_eq!(pv.unique_values(), 77);
    assert_eq!(pv.len(), iteration_count);
    for i in 0..iteration_count {
        assert_eq!(pv.get(i), Some(i as u32 % 77));
    }
    for i in 0..iteration_count {
        if (i % 77) % 3 == 0 {
            pv.set(i as usize, &0);
        }
    }
    assert_eq!(pv.unique_values(), 52);
    assert_eq!(pv.len(), iteration_count);
    pv.optimize();
    assert_eq!(pv.unique_values(), 52);
    assert_eq!(pv.len(), iteration_count);
    for i in 0..iteration_count {
        if (i % 77) % 3 == 0 {
            assert_eq!(pv.get(i), Some(0));
        } else {
            assert_eq!(pv.get(i), Some(i as u32 % 77));
        }
    }
    for i in 0..iteration_count {
        if (i % 77) % 2 == 0 {
            pv.set(i as usize, &0);
        }
    }
    assert_eq!(pv.unique_values(), 26);
    assert_eq!(pv.len(), iteration_count);
    pv.optimize();
    assert_eq!(pv.unique_values(), 26);
    assert_eq!(pv.len(), iteration_count);
    for i in 0..iteration_count {
        if ((i % 77) % 3) == 0 || ((i % 77) % 2) == 0 {
            assert_eq!(pv.get(i), Some(0));
        } else {
            assert_eq!(pv.get(i), Some(i as u32 % 77));
        }
    }
    for _ in 0..10 {
        pv.optimize();
    }
    assert_eq!(pv.unique_values(), 26);
    assert_eq!(pv.len(), iteration_count);
    for i in 0..iteration_count {
        if ((i % 77) % 3) == 0 || ((i % 77) % 2) == 0 {
            assert_eq!(pv.get(i), Some(0));
        } else {
            assert_eq!(pv.get(i), Some(i as u32 % 77));
        }
    }
}
