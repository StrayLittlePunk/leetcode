#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/increasing-triplet-subsequence/description/
//

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }
        let mut first = i32::MAX;
        let mut second = i32::MAX;
        for num in &nums {
            if *num <= first {
                first = *num;
            } else if *num <= second {
                second = *num;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_334() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    }
}
