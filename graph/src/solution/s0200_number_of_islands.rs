#![allow(unused)]
pub struct Solution {}

use std::collections::VecDeque;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ret = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] != '0' {
                    grid[row][col] = '0';
                    Self::bfs(&mut grid, row as i32, col as i32);
                    ret += 1;
                }
            }
        }

        ret
    }
    fn bfs(grid: &mut Vec<Vec<char>>, row: i32, col: i32) {
        let mut queue = VecDeque::new();
        queue.push_back((row, col));

        while let Some(node) = queue.pop_front() {
            for di in DIRECTIONS.iter() {
                let (next_row, next_col) = (di.0 + node.0, di.1 + node.1);
                if next_row >= 0
                    && next_row < grid.len() as i32
                    && next_col >= 0
                    && next_col < grid[0].len() as i32
                    && grid[next_row as usize][next_col as usize] == '1'
                {
                    queue.push_back((next_row, next_col));
                    grid[next_row as usize][next_col as usize] = '0';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_200() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );

        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ]),
            3
        );
    }
}
