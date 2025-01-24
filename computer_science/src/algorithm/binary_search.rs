/// Search through a sorted Vector for a target value.
///
/// Returns Some(index) of target value, or None if not found.
///
/// `list` should ideally be a custom struct (e.g. SortedVec) which guarantees that list is sorted. Not implemented for demo purposes.
pub fn binary_search<T>(list: &Vec<T>, target: T) -> Option<usize>
where
    T: Ord,
{
    if list.len() == 0 {
        return None;
    }

    let mut left: usize = 0;
    let mut right: usize = list.len() - 1;

    loop {
        if right - left <= 1 {
            if list[left] == target {
                return Some(left);
            };
            if list[right] == target {
                return Some(right);
            }
            return None;
        }

        let tested_position: usize = (left + right) / 2;

        if target == list[tested_position] {
            return Some(tested_position);
        } else if target < list[tested_position] {
            right = tested_position - 1;
        } else {
            left = tested_position + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_index() {
        let list = vec![1, 3, 8, 10, 19, 27];

        let index = binary_search(&list, 3);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn returns_none() {
        let list = vec![3, 5, 9];

        let index = binary_search(&list, 4);
        assert_eq!(index, None);
    }

    #[test]
    fn handles_empty() {
        let list = vec![];

        let index = binary_search(&list, 2);
        assert_eq!(index, None);
    }
}
