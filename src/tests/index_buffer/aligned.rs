use crate::index_buffer::aligned::AlignedIndexBuffer;

use super::*;

#[test]
fn index_buffer_set_index_size_no_mapping() {
    let mut buffer = AlignedIndexBuffer::new();
    for i in 1..64 {
        test_index_buffer_set_index_size_no_mapping(&mut AlignedIndexBuffer::new(), i);
        test_index_buffer_set_index_size_no_mapping(&mut buffer, i);
    }
    for i in (1..63).rev() {
        test_index_buffer_set_index_size_no_mapping(&mut buffer, i);
    }
}

#[test]
fn index_buffer_push() {
    let mut buffer = AlignedIndexBuffer::new();
    for index_size in 1..64 {
        test_index_buffer_push(&mut buffer, index_size, 1337);
        test_index_buffer_push(&mut AlignedIndexBuffer::new(), index_size, 1337);
    }
}

#[test]
fn index_buffer_pop() {
    let mut buffer = AlignedIndexBuffer::new();
    for index_size in 1..64 {
        test_index_buffer_pop(&mut buffer, index_size, 1337);
        test_index_buffer_pop(&mut AlignedIndexBuffer::new(), index_size, 1337);
    }
}

#[test]
fn index_buffer_set_index_size_growing() {
    let mut buffer = AlignedIndexBuffer::new();
    let mut index_sizes = (1..64).collect::<Vec<_>>();
    test_index_buffer_set_index_size_growing(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![3, 6, 8, 12, 15, 29, 45, 63];
    test_index_buffer_set_index_size_growing(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![50, 52, 54, 56, 58, 60];
    test_index_buffer_set_index_size_growing(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![2, 8, 16, 33];
    test_index_buffer_set_index_size_growing(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![1, 63];
    test_index_buffer_set_index_size_growing(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![1, 32];
    test_index_buffer_set_index_size_growing(&mut buffer, &mut index_sizes, 1337);
}

#[test]
fn index_buffer_set_index_size_shrinking() {
    let mut buffer = AlignedIndexBuffer::new();
    let mut index_sizes = (1..64).collect::<Vec<_>>();
    test_index_buffer_set_index_size_shrinking(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![3, 6, 8, 12, 15, 29, 45, 61, 62, 63];
    test_index_buffer_set_index_size_shrinking(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![50, 52, 54, 56, 58, 60];
    test_index_buffer_set_index_size_shrinking(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![2, 8, 16, 33];
    test_index_buffer_set_index_size_shrinking(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![1, 63];
    test_index_buffer_set_index_size_shrinking(&mut buffer, &mut index_sizes, 1337);

    let mut index_sizes = vec![1, 32];
    test_index_buffer_set_index_size_shrinking(&mut buffer, &mut index_sizes, 1337);
}

#[test]
fn index_buffer_get() {
    let mut buffer = AlignedIndexBuffer::new();
    for i in 1..64 {
        test_index_buffer_get(&mut AlignedIndexBuffer::new(), i, 1337);
        test_index_buffer_get(&mut buffer, i, 1337);
    }
}

#[test]
fn index_buffer_set() {
    let mut buffer = AlignedIndexBuffer::new();
    for i in 1..64 {
        test_index_buffer_set(&mut AlignedIndexBuffer::new(), i, 1337);
        test_index_buffer_set(&mut buffer, i, 1337);
    }
}

#[test]
fn index_buffer_index_size_0_operations() {
    let mut buffer = AlignedIndexBuffer::new();
    test_index_buffer_index_size_0_operations(&mut buffer);
}
