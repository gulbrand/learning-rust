fn smallest_window_to_sort_dumb(data: Vec<i32>) -> Result<(usize, usize), &'static str> {
    let a = data.as_slice();

    let mut left = 0;
    let mut right =
        match a.len().checked_sub(1) {
            Some(val) => val,
            None => return Err("already sorted")
        };
    let mut max = None;
    let mut min = None;
    // checked sub not needed, since done above
    let mut min_window_to_sort: (Option<usize>, Option<usize>) =
        (None, None);

    while left <= right {
        match max {
            None => max = Some(a[left]),
            Some(actual_max) => {
                if actual_max > a[left] {
                    min_window_to_sort.0 = Some(left - 1);
                    break;
                }
            }
        }
        left = match left.checked_add(1) {
            Some(value_after_add) => value_after_add,
            None => return Err("ran off the deep end"),
        };
    }

    while right > 0 {
        match min {
            None => min = Some(a[right]),
            Some(actual_min) => {
                if actual_min < a[right] {
                    min_window_to_sort.1 = Some(right + 1);
                    break;
                }
            }
        }
        right = match right.checked_sub(1) {
            Some(val) => val,
            None => return Err("ran off the shallow end"),
        }
    }

    if min_window_to_sort == (None, None) {
        return Err("already sorted");
    }
    let min_window_left = min_window_to_sort.0.unwrap_or(0);
    let min_window_right = min_window_to_sort.1.unwrap_or(a.len() - 1);

    return Ok((min_window_left, min_window_right));
}


fn smallest_window_to_sort(data: Vec<i32>)
                           -> Result<(usize, usize), &'static str> {
    let a = data.as_slice();

    match a.len().checked_sub(1) {
        None => return Err("already sorted"),
        _ => ()
    };

    let mut left: Option<usize> = None;
    let mut right: Option<usize> = None;

    let mut max_seen = std::i32::MIN;
    for i in 0..a.len() {
        max_seen = std::cmp::max(max_seen, a[i]);
        if a[i] < max_seen {
            right = Some(i);
        }
    }

    let mut min_seen = std::i32::MAX;
    for i in a.len() - 1..0 {
        min_seen = std::cmp::min(min_seen, a[i]);
        if a[i] > min_seen {
            left = Some(i);
        }
    }

    if left.is_none() && right.is_none() {
        return Err("already sorted");
    }

    return Ok(
        (
            left.unwrap_or(0),
            right.unwrap_or(a.len())
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let sample_data: Vec<i32> = [].to_vec();
        let window =
            smallest_window_to_sort(sample_data);

        assert_eq!(window, Err("already sorted"));
    }

    #[test]
    fn one_element() {
        let sample_data: Vec<i32> = [1].to_vec();
        let window =
            smallest_window_to_sort(sample_data);

        assert_eq!(window, Err("already sorted"));
    }

    #[test]
    fn two_elements_already_sorted() {
        let sample_data: Vec<i32> = [1, 2].to_vec();
        let window =
            smallest_window_to_sort(sample_data);

        assert_eq!(window, Err("already sorted"));
    }

    #[test]
    fn two_elements_not_sorted() {
        let sample_data: Vec<i32> = [2, 1].to_vec();
        let window =
            smallest_window_to_sort(sample_data);

        assert_eq!(window, Ok((0, 1)));
    }

    #[test]
    fn three_elements_not_sorted() {
        let sample_data: Vec<i32> = [3, 2, 1].to_vec();
        let window =
            smallest_window_to_sort(sample_data);

        assert_eq!(window, Ok((0, 2)));
    }

    #[test]
    fn three_elements_mostly_not_sorted() {
        let sample_data: Vec<i32> = [3, 1, 5].to_vec();
        let window =
            smallest_window_to_sort(sample_data);

        assert_eq!(window, Ok((0, 1)));
    }
}


fn main() {
    println!("this is to make rustc a happy camper");
}
