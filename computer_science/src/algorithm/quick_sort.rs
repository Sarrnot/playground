pub fn quick_sort<T: Ord>(list: &mut Vec<T>) {
    if list.len() <= 1 {
        return;
    }

    recursion(list, 0, list.len() - 1);
}

fn recursion<T: Ord>(list: &mut Vec<T>, low: usize, high: usize) {
    if low >= high {
        return;
    }

    let partition_index = create_partitions(list, low, high);

    if partition_index > 0 {
        recursion(list, low, partition_index - 1);
    }
    recursion(list, partition_index + 1, high);
}

fn create_partitions<T: Ord>(list: &mut Vec<T>, low: usize, high: usize) -> usize {
    let mut swap_index = low + 1;

    for j in (low + 1)..=high {
        if list[j] < list[low] {
            if swap_index != j {
                list.swap(swap_index, j);
            }
            swap_index += 1;
        }
    }

    let partition_index = swap_index - 1;
    if low != partition_index {
        list.swap(low, partition_index);
    }

    partition_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts() {
        let mut list = vec![4, 6, 2, 9, 1, 0, 3, 3];
        quick_sort(&mut list);

        let should_be_array = [0, 1, 2, 3, 3, 4, 6, 9];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn handles_empty() {
        let mut list: Vec<i32> = vec![];
        quick_sort(&mut list);
        assert!(list.is_empty());
    }

    #[test]
    fn handles_single_value() {
        let mut list: Vec<i32> = vec![1];
        quick_sort(&mut list);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], 1);
    }
}
