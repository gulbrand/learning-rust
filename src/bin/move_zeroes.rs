/*

Let's have to pointers with one left-to-right iteration.


if * == 0, move it forward
if ^ != 0, move it forward
if ^ == 0 and * != 0, swap ^ and *

0 1 0 3 12
^
*


0 1 0 3 12
^
  *

do the swap after the moves
1 0 0 3 12
^
  *

move the pointers, if conditions met
1 0 0 3 12
  ^
    *

no swap, loop, move the pointers, if conditions met
1 0 0 3 12
  ^
      *

do the swap after the moves and swap condition met
1 3 0 0 12
  ^
      *

move the pointers, if conditions met
1 3 0 0 12
    ^
        *

do the swap, if conditions are met
1 3 12 0 0
    ^
         *

move the pointers, if conditions are met
1 3 12 0 0
       ^
           *

* is off the end, terminate loop

*/
/*
if i == 0, move it forward
if i <= z, move it forward
if z != 0, move it forward
if i > z and i == 0 and z != 0, swap ^ and *

1 0 1
i
z
i
  z
  i
  z
    i
  z
1 1 0


move if con

*/

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut z = 0;
        if nums.len () < 2 {
            return;
        }
        loop {
            if i <= z || nums[i] == 0 {
                i += 1;
            }
            if nums[z] != 0 {
                z += 1;
            }
            if i >= nums.len() {
                return;
            }
            if nums[z] == 0 && nums[i] != 0 {
                nums.swap(i, z);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn simple_test() {
        let mut e = vec![1, 0];
        let expected = vec![1, 0];
        println!("{:?}", e);
        Solution::move_zeroes(&mut e);
        println!("{:?}", e);
        assert_eq!(e, expected);
    }

    #[test]
    pub fn less_simple_test() {
        let mut e = vec![1,0,1];
        let expected = vec![1, 1, 0];
        println!("{:?}", e);
        Solution::move_zeroes(&mut e);
        println!("{:?}", e);
        assert_eq!(e, expected);
    }


}

pub fn main() {
    println!("hello");
}