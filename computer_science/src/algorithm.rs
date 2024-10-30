pub fn binary_search<T>(list: Vec<T>, target: T) -> Result<usize, ()>
where
    T: Ord,
{
    if list.len() == 0 {
        return Err(());
    }

    let mut left: usize = 0;
    let mut right: usize = list.len() - 1;

    loop {
        if right - left <= 1 {
            if list[left] == target {
                return Ok(left);
            };
            if list[right] == target {
                return Ok(right);
            }
            return Err(());
        }

        let tested_position: usize = (left + right) / 2;

        if target == list[tested_position] {
            return Ok(tested_position);
        } else if target < list[tested_position] {
            right = tested_position - 1;
        } else {
            left = tested_position + 1;
        }
    }
}
