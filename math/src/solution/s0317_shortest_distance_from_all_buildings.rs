#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
use std::collections::VecDeque;
use std::i32::MAX;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let (m, n) = (grid.len(), grid[0].len());
        let (mut distance, mut reach) = (vec![vec![0; n]; m], vec![vec![0; n]; m]);

        let mut build_num = 0;

        for row in 0..m {
            for col in 0..n {
                if grid[row][col] == 1 {
                    build_num += 1;
                    let mut queue = VecDeque::new();
                    queue.push_back((row as i32, col as i32));

                    let mut is_visited = vec![vec![false; n]; m];
                    let mut level = 1;

                    while !queue.is_empty() {
                        let q_size = queue.len();
                        for q in 0..q_size {
                            if let Some(curr) = queue.pop_front() {
                                for di in DIRECTIONS.iter() {
                                    let next_row = curr.0 + di.0;
                                    let next_col = curr.1 + di.1;

                                    if next_row >= 0
                                        && next_row < m as i32
                                        && next_col >= 0
                                        && next_col < n as i32
                                        && grid[next_row as usize][next_col as usize] == 0
                                        && !is_visited[next_row as usize][next_col as usize]
                                    {
                                        // the shortest distance from [next_row][next_col] to 
                                        // this building is level
                                        distance[next_row as usize][next_col as usize] += level;
                                        reach[next_row as usize][next_col as usize] += 1;

                                        is_visited[next_row as usize][next_col as usize] = true;
                                        queue.push_back((next_row, next_col));
                                    }
                                }
                            }
                        }

                        level += 1;
                    }
                }
            }
        }

        let mut shortest = MAX;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 && reach[i][j] == build_num {
                    shortest = min(shortest, distance[i][j]);
                }
            }
        }

        if shortest == MAX {
            return -1;
        } else {
            return shortest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_317() {
        assert_eq!(
            Solution::shortest_distance(vec![
                vec![1, 0, 2, 0, 1],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0]
            ]),
            7
        );
    }
}
