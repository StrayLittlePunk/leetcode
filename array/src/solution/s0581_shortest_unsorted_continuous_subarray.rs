#![allow(unused)]
pub struct Solution {}

// uber interview
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut left = 0i32;
        for (i, n) in nums.iter().enumerate() {
            if nums[i] == sorted_nums[i] {
                left += 1;
            } else {
                break;
            }
        }
        
        let mut right = (nums.len() - 1) as i32;
        for j in 0..nums.len() {
            let i = nums.len() - j - 1;
            if nums[i] == sorted_nums[i] {
                right -= 1;
            } else {
                break;
            }
        }

        return if (right < left) { 0 } else { 1 + right - left }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_581() {
    }
}
