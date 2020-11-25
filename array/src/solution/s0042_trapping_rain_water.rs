#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/trapping-rain-water/description/
// https://leetcode.com/problems/trapping-rain-water/discuss/17357/Sharing-my-simple-c%2B%2B-code%3A-O(n)-time-O(1)-space
// Modeled on the previous c++ solution

use std::cmp::{max, min};
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
      if height.len() == 0 {
        return 0;
      }
      let mut l = 0;
      let mut r = height.len() - 1;
      let mut res = 0;
      let mut maxleft = 0;
      let mut maxright = 0;
      while l < r {
        if height[l] <= height[r] {
          if height[l] >= maxleft {
            maxleft = height[l];
          }else {
            res += maxleft - height[l];
          }
          l += 1;
        }else {
          if height[r] >= maxright {
            maxright = height[r];
          } else {
            res += maxright - height[r];
          }
          r -= 1;
        }
      }
      res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(
          Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
