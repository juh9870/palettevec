use crate::index_buffer::fast::FastIndexBuffer;

use super::*;

#[test]
fn index_buffer_set_index_size_no_mapping() {
    let mut buffer = FastIndexBuffer::new();
    for i in 1..64 {
        test_index_buffer_set_index_size_no_mapping(&mut FastIndexBuffer::new(), i);
        test_index_buffer_set_index_size_no_mapping(&mut buffer, i);
    }
    for i in (1..63).rev() {
        test_index_buffer_set_index_size_no_mapping(&mut buffer, i);
    }
}

#[test]
fn index_buffer_push() {
    let mut buffer = FastIndexBuffer::new();
    for index_size in 1..64 {
        test_index_buffer_push(&mut buffer, index_size, 1337);
        test_index_buffer_push(&mut FastIndexBuffer::new(), index_size, 1337);
    }
}

#[test]
fn index_buffer_pop() {
    let mut buffer = FastIndexBuffer::new();
    for index_size in 1..64 {
        test_index_buffer_pop(&mut buffer, index_size, 1337);
        test_index_buffer_pop(&mut FastIndexBuffer::new(), index_size, 1337);
    }
}

#[test]
fn index_buffer_set_index_size_growing() {
    let mut buffer = FastIndexBuffer::new();
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
    let mut buffer = FastIndexBuffer::new();
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
    let mut buffer = FastIndexBuffer::new();
    for i in 1..64 {
        test_index_buffer_get(&mut FastIndexBuffer::new(), i, 1337);
        test_index_buffer_get(&mut buffer, i, 1337);
    }
}

#[test]
fn index_buffer_set() {
    let mut buffer = FastIndexBuffer::new();
    for i in 1..64 {
        test_index_buffer_set(&mut FastIndexBuffer::new(), i, 1337);
        test_index_buffer_set(&mut buffer, i, 1337);
    }
}

#[test]
fn index_buffer_index_size_0_operations() {
    let mut buffer = FastIndexBuffer::new();
    test_index_buffer_index_size_0_operations(&mut buffer);
}

#[test]
fn index_buffer_len() {
    let mut buffer = FastIndexBuffer::new();
    test_index_buffer_len(&mut buffer, 3333);
}

#[test]
fn index_buffer_zeroed() {
    let mut buffer = FastIndexBuffer::new();
    for i in 0..64 {
        test_index_buffer_zeroed(&mut FastIndexBuffer::new(), i, 1337);
        test_index_buffer_zeroed(&mut buffer, i, 1337);
    }
}

#[test]
fn index_buffer_iterator() {
    for i in 0..64 {
        test_index_buffer_iterator(&mut FastIndexBuffer::new(), i, 1337);
    }
}
