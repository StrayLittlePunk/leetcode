pub struct Solution {}

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut cur = 0;
                dfs(&mut grid, i, j, &mut cur);
                ans = ans.max(cur);
            }
        }

        ans
    }
}

fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, cur: &mut i32) {
    if grid[i][j] == 0 {
        return;
    }

    grid[i][j] = 0;
    *cur += 1;
    if i >= 1 {
        dfs(grid, i - 1, j, cur)
    }
    if j >= 1 {
        dfs(grid, i, j - 1, cur)
    }
    if i + 1 < grid.len() {
        dfs(grid, i + 1, j, cur)
    }
    if j + 1 < grid[0].len() {
        dfs(grid, i, j + 1, cur)
    }
}

#[test]
fn testcase() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::max_area_of_island(grid), 6);
    let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
    assert_eq!(Solution::max_area_of_island(grid), 0);
}
