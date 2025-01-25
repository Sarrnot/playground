pub fn selection_sort<T>(list: &mut Vec<T>)
where
    T: Ord,
{
    if list.len() == 0 {
        return;
    }

    for i in 0..list.len() {
        let mut min_value_index = i;

        for j in (i + 1)..list.len() {
            if list[j] < list[min_value_index] {
                min_value_index = j;
            }
        }

        if min_value_index == i {
            continue;
        }

        list.swap(min_value_index, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts() {
        let mut list = vec![4, 6, 2, 9, 1, 0, 3, 3];
        selection_sort(&mut list);

        let should_be_array = [0, 1, 2, 3, 3, 4, 6, 9];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn handles_empty() {
        let mut list: Vec<i32> = vec![];
        selection_sort(&mut list);
        assert!(list.is_empty());
    }
}
