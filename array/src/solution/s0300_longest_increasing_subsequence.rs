#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/longest-increasing-subsequence/

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut d = vec![];
        d.push(nums[0]);

        for num in &nums {
            match d.binary_search(num) {
                Ok(n) => (),
                Err(n) => {
                    if n >= d.len() {
                        d.push(*num);
                    } else {
                        d[n] = *num;
                    }
                }
            }
        }
        d.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_300() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
