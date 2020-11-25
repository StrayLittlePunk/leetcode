#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/find-the-duplicate-number/description/

use std::cmp::max;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
      let mut slow: i32 = 0;
      let mut fast: i32 = 0;
      loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if slow == fast {
          break;
        }
      }
      slow = 0;
      while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
      }
      slow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_287() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
