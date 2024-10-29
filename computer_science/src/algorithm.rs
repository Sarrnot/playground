pub fn binary_search(list: Vec<i32>, target: i32) -> Result<usize, ()> {
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
        let tested_value: i32 = list[tested_position];

        if target == tested_value {
            return Ok(tested_position);
        } else if target < tested_value {
            right = tested_position - 1;
        } else {
            left = tested_position + 1;
        }
    }
}
