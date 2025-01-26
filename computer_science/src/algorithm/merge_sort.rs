pub fn merge_sort<T: Ord + Copy>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }

    let partition_index = list.len() / 2;

    let mut left = &mut list[0..partition_index].to_vec();
    let mut right = &mut list[partition_index..list.len()].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let mut merge_index = 0;
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        let left_value = left[left_index];
        let right_value = right[right_index];

        if left_value < right_value {
            list[merge_index] = left_value;
            left_index += 1;
        } else {
            list[merge_index] = right_value;
            right_index += 1;
        }

        merge_index += 1;
    }

    for i in left_index..left.len() {
        list[merge_index] = left[i];
        merge_index += 1;
    }

    for i in right_index..right.len() {
        list[merge_index] = right[i];
        merge_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts() {
        let mut list = vec![4, 6, 2, 9, 1, 0, 3, 3];
        merge_sort(&mut list);

        let should_be_array = [0, 1, 2, 3, 3, 4, 6, 9];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn handles_empty() {
        let mut list: Vec<i32> = vec![];
        merge_sort(&mut list);
        assert!(list.is_empty());
    }

    #[test]
    fn handles_single_value() {
        let mut list: Vec<i32> = vec![1];
        merge_sort(&mut list);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], 1);
    }
}
