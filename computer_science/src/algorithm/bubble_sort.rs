pub fn bubble_sort<T>(list: &mut Vec<T>)
where
    T: Ord,
{
    if list.len() == 0 {
        return;
    }

    for i in 0..list.len() {
        for j in 0..(list.len() - i - 1) {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts() {
        let mut list = vec![4, 6, 2, 9, 1, 0, 3, 3];
        bubble_sort(&mut list);

        let should_be_array = [0, 1, 2, 3, 3, 4, 6, 9];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn handles_empty() {
        let mut list: Vec<i32> = vec![];
        bubble_sort(&mut list);
        assert!(list.is_empty());
    }

    #[test]
    fn handles_single_value() {
        let mut list: Vec<i32> = vec![1];
        bubble_sort(&mut list);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], 1);
    }
}
