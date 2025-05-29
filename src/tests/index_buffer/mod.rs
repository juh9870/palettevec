use std::u64;

use crate::index_buffer::IndexBuffer;

mod aligned;

fn test_index_buffer_set_index_size_no_mapping<B: IndexBuffer>(buffer: &mut B, index_size: usize) {
    buffer.set_index_size(index_size, None);
}

fn test_index_buffer_push<B: IndexBuffer>(
    buffer: &mut B,
    index_size: usize,
    iteration_count: usize,
) {
    buffer.set_index_size(index_size, None);
    for i in 0..iteration_count {
        buffer.push_index(i);
    }
}

fn test_index_buffer_pop<B: IndexBuffer>(
    buffer: &mut B,
    index_size: usize,
    iteration_count: usize,
) {
    assert_eq!(buffer.pop_index(), None);
    buffer.set_index_size(index_size, None);
    for i in 0..iteration_count {
        let index = i & (usize::MAX >> (64 - index_size));
        buffer.push_index(index);
        assert_eq!(buffer.pop_index(), Some(index));
        assert_eq!(buffer.pop_index(), None);
    }
    for i in 0..iteration_count {
        let index = i & (usize::MAX >> (64 - index_size));
        buffer.push_index(index);
    }
    for i in (0..iteration_count).rev() {
        let index = i & (usize::MAX >> (64 - index_size));
        assert_eq!(buffer.pop_index(), Some(index));
    }
    assert_eq!(buffer.pop_index(), None);
}

fn test_index_buffer_set_index_size_growing<B: IndexBuffer>(
    buffer: &mut B,
    index_sizes: &mut Vec<usize>,
    iteration_count: usize,
) {
    assert_eq!(buffer.pop_index(), None);
    index_sizes.sort();

    let lowest_index_size = index_sizes[0];
    buffer.set_index_size(lowest_index_size, None);
    for i in 0..iteration_count {
        let index = i & (usize::MAX >> (64 - lowest_index_size));
        buffer.push_index(index);
    }

    for index_size in index_sizes.iter().skip(1) {
        buffer.set_index_size(*index_size, None);
        for i in (0..iteration_count).rev() {
            let index = i & (usize::MAX >> (64 - lowest_index_size));
            assert_eq!(buffer.pop_index(), Some(index));
        }
        assert_eq!(buffer.pop_index(), None);
        for i in 0..iteration_count {
            let index = i & (usize::MAX >> (64 - lowest_index_size));
            buffer.push_index(index);
        }
    }

    for i in (0..iteration_count).rev() {
        let index = i & (usize::MAX >> (64 - lowest_index_size));
        assert_eq!(buffer.pop_index(), Some(index));
    }
    assert_eq!(buffer.pop_index(), None);
}

fn test_index_buffer_set_index_size_shrinking<B: IndexBuffer>(
    buffer: &mut B,
    index_sizes: &mut Vec<usize>,
    iteration_count: usize,
) {
    assert_eq!(buffer.pop_index(), None);
    index_sizes.sort();
    index_sizes.reverse();

    let highest_index_size = index_sizes[0];
    buffer.set_index_size(highest_index_size, None);
    for _ in 0..iteration_count {
        buffer.push_index(1);
    }

    for index_size in index_sizes.iter().skip(1) {
        buffer.set_index_size(*index_size, None);
        for _ in 0..iteration_count {
            assert_eq!(buffer.pop_index(), Some(1));
        }
        assert_eq!(buffer.pop_index(), None);
        for _ in 0..iteration_count {
            buffer.push_index(1);
        }
    }

    for _ in 0..iteration_count {
        assert_eq!(buffer.pop_index(), Some(1));
    }
    assert_eq!(buffer.pop_index(), None);
}

fn test_index_buffer_get<B: IndexBuffer>(
    buffer: &mut B,
    index_size: usize,
    iteration_count: usize,
) {
    assert_eq!(buffer.pop_index(), None);
    buffer.set_index_size(index_size, None);
    let possible_different_indices = 2 << (index_size - 1);
    for i in 0..iteration_count {
        let index = i % possible_different_indices;
        buffer.push_index(index);
    }
    for i in 0..iteration_count {
        assert_eq!(buffer.get_index(i as usize), i % possible_different_indices);
    }
    for i in (0..iteration_count).rev() {
        assert_eq!(buffer.pop_index(), Some(i % possible_different_indices));
    }
    assert_eq!(buffer.pop_index(), None);
}

fn test_index_buffer_set<B: IndexBuffer>(
    buffer: &mut B,
    index_size: usize,
    iteration_count: usize,
) {
    assert_eq!(buffer.pop_index(), None);
    buffer.set_index_size(index_size, None);
    let possible_different_indices = 2 << (index_size - 1);
    for i in 0..iteration_count {
        let index = i % possible_different_indices;
        buffer.push_index(index);
    }
    for i in 0..iteration_count {
        let index = (i + 1) % possible_different_indices;
        assert_eq!(
            buffer.set_index(i as usize, index),
            i % possible_different_indices
        );
    }
    for i in (0..iteration_count).rev() {
        let index = (i + 1) % possible_different_indices;
        assert_eq!(buffer.pop_index(), Some(index));
    }
    assert_eq!(buffer.pop_index(), None);
}

fn test_index_buffer_index_size_0_operations<B: IndexBuffer>(buffer: &mut B) {
    assert_eq!(buffer.pop_index(), None);
    buffer.set_index_size(0, None);
    assert_eq!(buffer.pop_index(), None);
    for _ in 0..10 {
        buffer.push_index(0);
    }
    for i in 0..10 {
        assert_eq!(buffer.get_index(i), 0);
    }
    // Setting is not tested because we should never set anyway when index size is 0
    for _ in 0..10 {
        assert_eq!(buffer.pop_index(), Some(0));
    }
    for _ in 0..10 {
        buffer.push_index(0);
    }
    buffer.set_index_size(1, None);
    for i in 0..10 {
        assert_eq!(buffer.get_index(i), 0);
    }
    buffer.set_index_size(0, None);
    for i in 0..10 {
        assert_eq!(buffer.get_index(i), 0);
    }
    for _ in 0..10 {
        assert_eq!(buffer.pop_index(), Some(0));
    }
    assert_eq!(buffer.pop_index(), None);
}

fn test_index_buffer_len<B: IndexBuffer>(buffer: &mut B, iteration_count: usize) {
    assert_eq!(buffer.pop_index(), None);
    assert_eq!(buffer.len(), 0);
    for i in 1..iteration_count + 1 {
        buffer.push_index(0);
        assert_eq!(buffer.len(), i);
    }
    assert_eq!(buffer.len(), iteration_count);
    for i in (0..iteration_count).rev() {
        buffer.pop_index();
        assert_eq!(buffer.len(), i);
    }
}

fn test_index_buffer_zeroed<B: IndexBuffer>(
    buffer: &mut B,
    index_size: usize,
    iteration_count: usize,
) {
    assert_eq!(buffer.pop_index(), None);
    assert_eq!(buffer.len(), 0);
    buffer.set_index_size(index_size, None);
    buffer.zeroed(iteration_count);
    for _ in 0..iteration_count {
        assert_eq!(buffer.pop_index(), Some(0));
    }
    assert_eq!(buffer.pop_index(), None);
    assert_eq!(buffer.len(), 0);
    for _ in 0..iteration_count {
        buffer.push_index(1);
    }
    buffer.zeroed(iteration_count);
    for _ in 0..iteration_count {
        assert_eq!(buffer.pop_index(), Some(0));
    }
    assert_eq!(buffer.pop_index(), None);
    assert_eq!(buffer.len(), 0);
}
