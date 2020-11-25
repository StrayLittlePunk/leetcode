#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] > 0 {
                    ret = max(
                        ret,
                        Self::dfs(&mut grid, row as i32, col as i32) + grid[row][col],
                    )
                }
            }
        }

        return ret;
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
        let orginal = grid[row as usize][col as usize];
        grid[row as usize][col as usize] = 0;
        let mut ret = 0;

        for di in DIRECTIONS.iter() {
            let (next_row, next_col) = (row + di.0, col + di.1);
            if next_row >= 0
                && next_row < grid.len() as i32
                && next_col >= 0
                && next_col < grid[0].len() as i32
                && grid[next_row as usize][next_col as usize] != 0
            {
                ret = max(
                    ret,
                    grid[next_row as usize][next_col as usize]
                        + Self::dfs(grid, next_row, next_col),
                );
            }
        }

        grid[row as usize][col as usize] = orginal;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1219() {}
}
