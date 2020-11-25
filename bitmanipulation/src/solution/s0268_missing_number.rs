#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(N) O(1)
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut missing = nums.len();
        for i in 0..nums.len() {
            missing ^= i ^ nums[i] as usize;
        }
        missing as i32
    }
    pub fn missing_number_gauss(nums: Vec<i32>) -> i32 {
        let expected_sum = nums.len() * (nums.len() + 1) / 2;

        let mut actual_sum = 0;
        for num in nums {
            actual_sum += num;
        }

        expected_sum as i32 - actual_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_268() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }
}
