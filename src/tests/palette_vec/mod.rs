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

fn test_palette_vec_push_pop<P, B>()
where
    P: Palette<i32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<i32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);

    // Push
    for _ in 0..3333 {
        pv.push(1);
    }
    for _ in (0..3333).rev() {
        assert_eq!(pv.pop(), Some(1));
    }
    assert_eq!(pv.pop(), None);
    for i in 0..3333 {
        pv.push(i % 69);
    }
    for i in (0..3333).rev() {
        assert_eq!(pv.pop(), Some(i % 69));
    }
    assert_eq!(pv.pop(), None);
}

fn test_palette_vec_push_ref_pop<P, B>()
where
    P: Palette<i32>,
    B: IndexBuffer,
{
    let mut pv: PaletteVec<i32, P, B> = PaletteVec::new();
    assert_eq!(pv.pop(), None);

    // Push_ref
    for _ in 0..3333 {
        pv.push_ref(&1);
    }
    for _ in (0..3333).rev() {
        assert_eq!(pv.pop(), Some(1));
    }
    assert_eq!(pv.pop(), None);
    for i in 0..3333 {
        pv.push_ref(&(i % 69));
    }
    for i in (0..3333).rev() {
        assert_eq!(pv.pop(), Some(i % 69));
    }
    assert_eq!(pv.pop(), None);
}
