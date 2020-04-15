/**
Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.

For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].

Follow-up: what if you can't use division?
**/

// What type ? Are these integers? Floats?
// Positive and negative alike?
// Max value?
// Max length?

// Solution 1
// Fold the multiply operator over the array
//fn product_in_place(elements: &Vec<i32>) -> Vec<i32> {
//    let max_product = elements.iter().fold(1, |product, i| product * i);
//    elements.iter().map(|i| product / i)
////    elements.iter().map(|i| productr / i).as_slice()
//
//
//}

fn main() {
    println!("hello");
}