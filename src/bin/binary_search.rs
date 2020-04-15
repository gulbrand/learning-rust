// given an array and a target value, find it.

use std::slice::SliceIndex;

fn find_by_recursion(
    slice: &[i32],
    target_value: i32,
    lower_bound: usize,
    upper_bound: usize) -> Result<usize, &'static str> {
    if slice.len() < 1 {
        return Err("not found");
    }
    let mid = (lower_bound + upper_bound) / 2;
    if lower_bound == upper_bound {
        if slice[mid] == target_value {
            return Ok(mid);
        } else {
            return Err("not found");
        }
    }
    if target_value < slice[mid] {
        return
            find_by_recursion(
                slice,
                target_value,
                lower_bound,
                mid);
    } else if target_value > slice[mid] {
        return
            find_by_recursion(
                slice,
                target_value,
                mid,
                upper_bound);
    }
    return Ok(mid);
}

fn find_by_loop(
    slice: &[i32],
    target_value: i32) -> Result<usize, &'static str> {
    if slice.len() < 1 {
        return Err("not found");
    }
    let mut left = 0;
    let mut right = slice.len() - 1;
    while left <= right {
        let pivot = left + (right - left) / 2;
        if slice[pivot] == target_value {
            return Ok(pivot);
        }
        if target_value < slice[pivot] {
            right = match pivot.checked_sub(1) {
                Some(val) => val,
                None => return Err("not found"),
            };
        } else {
            left = pivot + 1;
        }
    }
    Err("not found")
}


fn find(
    slice: &[i32],
    target_value: i32,
    lower_bound: usize,
    upper_bound: usize) -> Result<usize, &'static str> {
    find_by_loop(slice, target_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let sample_data: [i32; 0] = [];
        let actual = find(
            &sample_data,
            0,
            0,
            sample_data.len(),
        );
        assert_eq!(actual, Err("not found"));
    }

    #[test]
    fn single_element_not_found() {
        let sample_data: [i32; 1] = [1];
        let actual = find(
            &sample_data,
            0,
            0,
            sample_data.len(),
        );
        assert_eq!(actual, Err("not found"));
    }

    #[test]
    fn single_element_found() {
        let sample_data: [i32; 1] = [1];
        let target_value = 1;
        let actual = find(
            &sample_data,
            target_value,
            0,
            sample_data.len(),
        );
        assert_eq!(actual, Ok(0));
    }

    #[test]
    fn multiple_element_found_at_last_index() {
        let sample_data: [i32; 6] = [1, 2, 3, 4, 100, 300];
        let target_value = 300;
        let actual = find(
            &sample_data,
            target_value,
            0,
            sample_data.len(),
        );
        assert_eq!(actual, Ok(5));
    }

    #[test]
    fn multiple_element_found_at_first_index() {
        let sample_data: [i32; 6] = [1, 2, 3, 4, 100, 300];
        let target_value = 1;
        let actual = find(
            &sample_data,
            target_value,
            0,
            sample_data.len(),
        );
        assert_eq!(actual, Ok(0));
    }

    #[test]
    fn multiple_element_found_at_mid_index() {
        let sample_data: [i32; 7] = [1, 2, 3, 4, 100, 300, 900];
        let target_value = 4;
        let actual = find(
            &sample_data,
            target_value,
            0,
            sample_data.len(),
        );
        assert_eq!(actual, Ok(3));
    }


    #[test]
    fn as_vec() {
        let sample_data: Vec<i32> =
            [1, 2, 3, 4, 100, 300, 900].to_vec();
        let target_value = 4;
        let actual =
            find(
                sample_data.as_slice(),
                target_value,
                0,
                sample_data.len(),
            );
        assert_eq!(actual, Ok(3));
    }

    #[test]
    fn leet_code_failed() {
        let sample_data: Vec<i32> =
            [2, 5].to_vec();
        let target_value = 0;
        let actual =
            find(sample_data.as_slice(), 0,
                 0, sample_data.len());
        assert_eq!(actual, Err("not found"));
    }
}

fn main() {
    println!("Hi");
}