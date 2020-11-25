#![allow(unused)]
pub struct Solution {}

// https://leetcode-cn.com/problems/container-with-most-water/

use std::cmp::{max, min};
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        while (l < r) {
            max_area = max(max_area, min(height[l], height[r]) * (r - l) as i32);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
