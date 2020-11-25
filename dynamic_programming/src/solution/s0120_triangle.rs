#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
impl Solution {
    // top-> down
    pub fn minimum_total_td(mut triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() < 1 || triangle[0].len() < 1 {
            return 0;
        }
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    triangle[i][j] = triangle[i - 1][j] + triangle[i][j];
                } else {
                    if j < triangle[i - 1].len() {
                        triangle[i][j] =
                            min(triangle[i - 1][j - 1], triangle[i - 1][j]) + triangle[i][j];
                    } else {
                        triangle[i][j] = triangle[i - 1][j - 1] + triangle[i][j];
                    }
                }
            }
        }

        *triangle[triangle.len() - 1].iter().min().unwrap()
    }

    // bottom-> up
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() < 1 || triangle[0].len() < 1 {
            return 0;
        }

        for row in (0..triangle.len() - 1).rev() {
            for col in 0..triangle[row].len() {
                triangle[row][col] =
                    min(triangle[row + 1][col], triangle[row + 1][col + 1]) + triangle[row][col];
            }
        }

        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
