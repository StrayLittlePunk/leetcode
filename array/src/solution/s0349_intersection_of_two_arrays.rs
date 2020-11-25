#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
      let mut h1: HashSet<i32> = nums1.into_iter().collect();
      let mut h2: HashSet<i32> = nums2.into_iter().collect();

      // cloned() is needed
       h1.intersection(&h2).cloned().collect()

      // another way to return vector
      /* let mut res: Vec<i32> = Vec::new();
       * for x in h1.intersection(&h2) {
       *   res.push(*x);
       * }
       * res */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_349() {
        assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}
