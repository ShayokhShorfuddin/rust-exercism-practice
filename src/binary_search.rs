pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut array = array.to_vec();

    if array.is_empty() {
        return None;
    }

    array.sort();

    // If the number is out of bound
    if key < array[0] || key > array[array.len() - 1] {
        return None;
    }

    let mut left_idx_pointer: usize = 0;
    let mut right_idx_pointer: usize = (array.len()) - 1;

    while left_idx_pointer != right_idx_pointer {
        // Need to get the middle index
        let middle_index: usize;

        if (left_idx_pointer + right_idx_pointer) % 2 == 0 {
            middle_index = (left_idx_pointer + right_idx_pointer) / 2;
        } else {
            middle_index = ((left_idx_pointer + right_idx_pointer) - 1) / 2;
        }

        if key > array[middle_index] {
            left_idx_pointer = middle_index + 1
        }

        if key < array[middle_index] {
            right_idx_pointer = middle_index - 1
        }

        if key == array[middle_index] {
            return Some(middle_index);
        }
    }

    if array[left_idx_pointer] != key {
        None
    } else {
        Some(left_idx_pointer)
    }
}
