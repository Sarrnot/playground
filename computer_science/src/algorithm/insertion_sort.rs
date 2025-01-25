pub fn insertion_sort<T>(list: &mut Vec<T>)
where
    T: Ord,
{
    for i in 1..list.len() {
        for j in (1..=i).rev() {
            if list[j] >= list[j - 1] {
                break;
            }
            list.swap(j, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts() {
        let mut list = vec![4, 6, 2, 9, 1, 0, 3, 3];
        insertion_sort(&mut list);

        let should_be_array = [0, 1, 2, 3, 3, 4, 6, 9];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn handles_empty() {
        let mut list: Vec<i32> = vec![];
        insertion_sort(&mut list);
        assert!(list.is_empty());
    }

    #[test]
    fn handles_single_value() {
        let mut list: Vec<i32> = vec![1];
        insertion_sort(&mut list);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], 1);
    }
}
