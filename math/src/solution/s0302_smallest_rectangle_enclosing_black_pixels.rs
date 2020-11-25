#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let (mut top, mut bottom, mut left, mut right) = (x, x, y, y);
        for x in 0..image.len() {
            for y in 0..image[0].len() {
                if image[x][y] == '1' {
                    top = min(top, x as i32);
                    bottom = max(bottom, x as i32 + 1);
                    left = min(left, y as i32);
                    right = max(right, y as i32 + 1);
                }
            }
        }

        (right - left) * (bottom - top)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_302() {
        assert_eq!(
            Solution::min_area(
                vec![
                    vec!['0', '0', '1', '0'],
                    vec!['0', '1', '1', '0'],
                    vec!['0', '1', '0', '0']
                ],
                0,
                2
            ),
            6
        );
    }
}
