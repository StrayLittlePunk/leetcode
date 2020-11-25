#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
use std::usize::MAX;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
impl Solution {
    // O(m * n) O(m * n)
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        let mut cache = vec![vec![0; n]; m];
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                ans = max(ans, Self::dfs(&matrix, i, j, &mut cache));
            }
        }

        ans
    }

    fn dfs(matrix: &Vec<Vec<i32>>, i: usize, j: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[i][j] != 0 {
            return cache[i][j];
        }

        for di in DIRECTIONS.iter() {
            let x = (i as i32 + di.0) as usize;
            let y = (j as i32 + di.1) as usize;
            if x != MAX
                && x < matrix.len()
                && y != MAX
                && y < matrix[0].len()
                && matrix[x][y] > matrix[i][j]
            {
                cache[i][j] = max(cache[i][j], Self::dfs(matrix, x, y, cache));
            }
        }

        cache[i][j] += 1;

        return cache[i][j];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_329() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );

        assert_eq!(
            Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
    }
}
