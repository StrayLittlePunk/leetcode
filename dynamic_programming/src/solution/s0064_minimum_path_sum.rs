#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
use std::i32::MAX;
impl Solution {

    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        grid.insert(0, vec![MAX; n + 1]);
        for row in 1..=m {
            grid[row].insert(0, MAX);
        }


        for row in 1..=m {
            for col in 1..=n {
                if min(grid[row - 1][col], grid[row][col - 1]) == MAX {
                    continue;
                }
                grid[row][col] = min(grid[row - 1][col], grid[row][col - 1]) + grid[row][col];
            }
        }

        grid[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
