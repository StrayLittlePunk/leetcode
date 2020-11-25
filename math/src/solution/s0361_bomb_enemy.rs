#![allow(unused)]
pub struct Solution {}

use std::cmp::max;

impl Solution {
  // O(mn(n + m)) O(1)
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let mut ans = -1;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                ans = max(ans, Self::detect_enemies(row, col, &grid));
            }
        }

        ans
    }

    fn detect_enemies(row: usize, col: usize, grid: &Vec<Vec<char>>) -> i32 {
        if grid[row][col] != '0' {
            return 0;
        }

        let mut ret = 0;

        // check row's enemies
        for i in (0..row).rev() {
            if grid[i][col] == 'W' {
                break;
            } else if grid[i][col] == 'E' {
                ret += 1;
            }
        }
        for i in row..grid.len() {
            if grid[i][col] == 'W' {
                break;
            } else if grid[i][col] == 'E' {
                ret += 1;
            }
        }
        // check column enemies

        for i in (0..col).rev() {
            if grid[row][i] == 'W' {
                break;
            } else if grid[row][i] == 'E' {
                ret += 1;
            }
        }
        for i in col..grid[row].len() {
            if grid[row][i] == 'W' {
                break;
            } else if grid[row][i] == 'E' {
                ret += 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_361() {
        assert_eq!(
            Solution::max_killed_enemies(vec![
                vec!['0', 'E', '0', '0'],
                vec!['E', '0', 'W', 'E'],
                vec!['0', 'E', '0', '0']
            ]),
            3
        );
    }
}
