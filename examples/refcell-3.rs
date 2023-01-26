// #![allow(unused)]
// fn main() {
// fn is_even(i: i32) -> bool {
//     i % 2 == 0
// }

// fn retain_even(nums: &mut Vec<i32>) {
//     let mut i = 0;
//     for num in nums.iter().filter(|&num| is_even(*num)) {
//         nums[i] = *num;
//         i += 1;
//     }
//     nums.truncate(i);
// }

#![allow(unused)]
fn main() {
    fn is_even(i: i32) -> bool {
        i % 2 == 0
    }
    fn retain_even(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if is_even(nums[j]) {
                nums[i] = nums[j];
                i += 1;
            }
        }
        nums.truncate(i);
    }
}

// #![allow(unused)]
// fn main() {
// use std::cell::Cell;

// fn retain_even(nums: &mut Vec<i32>) {
//     let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
//         .as_slice_of_cells();

//     let mut i = 0;
//     for num in slice.iter().filter(|num| is_even(num.get())) {
//         slice[i].set(num.get());
//         i += 1;
//     }

//     nums.truncate(i);
// }
// }
