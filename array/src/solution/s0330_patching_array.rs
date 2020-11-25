#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/min_patches/description/
//
use std::cmp::max;
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
      let mut patches = 0;
      let mut i = 0;
      let mut miss: i64 = 0;
      while miss <= (n as i64) {
        if i < nums.len() && (nums[i] as i64) <= miss {
          miss += (nums[i] as i64) ;
          i += 1;
        } else {
          miss += miss;
          patches+=1;
        }
      }
      patches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_330() {
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
        assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
    }
}
