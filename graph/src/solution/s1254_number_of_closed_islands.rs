#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n, mut ans) = (grid.len(), grid[0].len(), 0);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    if Self::dfs(&mut grid, i as i32, j as i32, m, n) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, m: usize, n: usize) -> bool {
        if i < 0 || j < 0 || i >= m as i32 || j >= n as i32 {
            return false;
        }
        if grid[i as usize][j as usize] == 1 {
            return true;
        }
        grid[i as usize][j as usize] = 1;

        let top = Self::dfs(grid, i - 1, j, m, n);
        let down = Self::dfs(grid, i + 1, j, m, n);
        let left = Self::dfs(grid, i, j - 1, m, n);
        let right = Self::dfs(grid, i, j + 1, m, n);

        top && down && left && right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1254() {}
}
