#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/candy/description/
//
use std::cmp::max;
impl Solution {
    pub fn count(n: i32) -> i32 {
      (n * (n + 1)) / 2
    }
    pub fn candy(ratings: Vec<i32>) -> i32 {
      if ratings.len() < 2 {
        return ratings.len() as i32
      }
      let mut candies = 0;
      let mut up = 0;
      let mut down = 0;
      let mut old_slope = 0;
      for i in 1..ratings.len() {
        let mut new_slope = if ratings[i] > ratings[i -1] {
          1
        }else if ratings[i] < ratings [i -1] {
          -1
        } else {
          0
        };
        if (old_slope > 0 && new_slope == 0 ) || (old_slope < 0 && new_slope >= 0) {
          candies += Solution::count(up) + Solution::count(down) + max(up, down);
          up = 0;
          down = 0;
        }
        if new_slope > 0 {
          up += 1;
        } else if new_slope < 0 {
          down += 1;
        } else {
          candies += 1;
        }
        old_slope = new_slope;
      }
      candies += Solution::count(up) + Solution::count(down) + max(up , down) +1;
      candies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_135() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
}
