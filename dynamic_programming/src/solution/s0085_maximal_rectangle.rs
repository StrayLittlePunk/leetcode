#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return 0;
        }
        let (rows, cols) = (matrix.len(), matrix[0].len());

        let mut left = vec![0; cols];
        let mut right = vec![cols as i32; cols];
        let mut height = vec![0; cols];

        let mut max_area = 0;

        for i in 0..rows {
            let (mut cur_left, mut cur_right) = (0, cols as i32);
            // update height
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    height[j] += 1;
                } else {
                    height[j] = 0;
                }
            }

            // update left
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    left[j] = max(left[j], cur_left);
                } else {
                    left[j] = 0;
                    cur_left = j as i32 + 1;
                }
            }

            // update right
            for j in (0..cols).rev() {
                if matrix[i][j] == '1' {
                    right[j] = min(right[j], cur_right);
                } else {
                    right[j] = cols as i32;
                    cur_right = j as i32;
                }
            }

            for j in 0..cols {
                max_area = max(max_area, height[j] * (right[j] - left[j]));
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_85() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}
