#![allow(unused)]

pub struct Solution {}
// https://leetcode.com/problems/maximum-product-subarray/description/

use std::cmp::{max,min};
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
      
 let mut answer = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let n = nums[i];
            while i < nums.len() - 1 && nums[i + 1] == nums[i] + 1 {
                i += 1;
            }
            let mut s = n.to_string();
            if n != nums[i] {
                s += "->";
                s += &nums[i].to_string();
            };
            answer.push(s);
            i += 1;
        }
        answer

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_228() {
        assert_eq!(
          Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]), 
            vec!["0->2", "4->5", "7"]);
        assert_eq!(
          Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), 
            vec!["0","2->4", "6", "8->9"]);
    }
}
