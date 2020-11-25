#![allow(unused)]
pub struct Solution {}

use std::i32::{MAX,MIN};
use std::cmp::{min, max};
use std::collections::HashSet;
impl Solution {
    pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
      let mut max_val = MIN;
      let mut min_val = MAX;

      let mut set = HashSet::new();

      for p in points.iter() {
        max_val = max(max_val, p[0]);
        min_val = min(min_val, p[0]);
        set.insert((p[0], p[1]));
      }

      let sum = max_val + min_val;
      for p in points.iter() {
        if set.get(&(sum - p[0], p[1])).is_none() {
          return false;
        }
      }

      true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_356() {
        // code here
        assert_eq!(Solution::is_reflected(vec![vec![1, 1], vec![-1, 1]]), true);
        assert_eq!(Solution::is_reflected(vec![vec![1, 1], vec![-1, -1]]), false);
    }
}
