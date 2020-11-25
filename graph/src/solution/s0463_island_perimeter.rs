#![allow(unused)]
pub struct Solution {}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (m, n) = (grid.len(), grid[0].len());
        for row in 0..m {
            for col in 0..n {
                if grid[row][col] != 1 {
                    continue;
                }

                for di in DIRECTIONS.iter() {
                    let (next_row, next_col) = (row as i32 + di.0, col as i32 + di.1);
                    if next_row < 0 || next_col < 0 || next_row >= m as i32 || next_col >= n as i32
                    {
                        ans += 1;
                    } else if grid[next_row as usize][next_col as usize] == 0 {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_463() {}
}
