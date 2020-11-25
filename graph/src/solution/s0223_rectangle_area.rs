#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let left = max(a, e);
        let right = max(min(c, g), left);
        let bottom = max(b, f);
        let top = max(min(d, h), bottom);
        return (c - a) * (d - b) - (right - left) * (top - bottom) + (g - e) * (h - f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_223() {
        // code here
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }
}
