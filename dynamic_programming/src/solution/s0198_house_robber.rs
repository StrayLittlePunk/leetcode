#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        for i in 2..nums.len() {
            nums[i] = nums[i] + *(&nums[0..i - 1].iter().max().unwrap());
        }

        *nums.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_198() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
