#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        obstacle_grid[0][0] = 1;

        for i in 1..m {
            obstacle_grid[i][0] = if obstacle_grid[i][0] == 0 && obstacle_grid[i - 1][0] == 1 {
                1
            } else {
                0
            };
        }
        for j in 1..n {
            obstacle_grid[0][j] = if obstacle_grid[0][j] == 0 && obstacle_grid[0][j - 1] == 1 {
                1
            } else {
                0
            };
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 0 {
                    obstacle_grid[i][j] = obstacle_grid[i - 1][j] + obstacle_grid[i][j - 1];
                } else {
                    obstacle_grid[i][j] = 0;
                }
            }
        }

        obstacle_grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_63() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
    }
}
