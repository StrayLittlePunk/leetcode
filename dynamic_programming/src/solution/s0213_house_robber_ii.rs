#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        max(
            Self::rob_simple(&nums, 0, nums.len() - 2),
            Self::rob_simple(&nums, 1, nums.len() - 1),
        )
    }

    fn rob_simple(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let (mut t1, mut t2) = (0, 0);
        for i in start..=end {
            let tmp = t1;
            t1 = max(nums[i] + t2, t1);
            t2 = tmp;
        }

        t1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_213() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![0]), 0);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
