// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// Example:
//
// Given nums = [2, 7, 11, 15], target = 9,
//
// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].


pub struct Solution {}


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.iter().enumerate() {
            if let Some(sub_index) = map.get(&(target - *num)) {
                return vec![*sub_index as i32, idx as i32];
            }
            map.insert(num, idx);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn test_raw_pointers(){
        let mut num = 1;
        // 将引用转为裸指针
        let num_raw_point = &mut num as *mut i32;
        unsafe {
            *num_raw_point = 100;
            println!("{} {} {:p}", num, *num_raw_point, &num);
            // Output: 100 100 0x8d8c6ff6bc
        }

        let address = num_raw_point as usize;
        // 将一个 usize 对象，转化为 裸指针
        let raw = address as *mut i32;
        unsafe {
            *raw = 200;
            println!("{} {} {:p} {}", num, *num_raw_point, &num, address);
            // Output: 200 200 0x8d8c6ff6bc 607946536636
        }
    }
}