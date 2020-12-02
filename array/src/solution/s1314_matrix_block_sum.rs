#![allow(unused)]
pub struct Solution {}

/* +-----+-+-------+     +--------+-----+     +-----+---------+     +-----+--------+
 * |     | |       |     |        |     |     |     |         |     |     |        |
 * |     | |       |     |        |     |     |     |         |     |     |        |
 * +-----+-+       |     +--------+     |     |     |         |     +-----+        |
 * |     | |       |  =  |              |  +  |     |         |  -  |              | + mat[i][j]
 * +-----+-+       |     |              |     +-----+         |     |              |
 * |               |     |              |     |               |     |              |
 * |               |     |              |     |               |     |              |
 * +---------------+     +--------------+     +---------------+     +--------------+
 *
 * rangeSum[i+1][j+1] =  rangeSum[i][j+1] + rangeSum[i+1][j]    -   rangeSum[i][j]   + mat[i][j]
 *
 * +---------------+   +--------------+   +---------------+   +--------------+   +--------------+
 * |               |   |         |    |   |   |           |   |         |    |   |   |          |
 * |   (r1,c1)     |   |         |    |   |   |           |   |         |    |   |   |          |
 * |   +------+    |   |         |    |   |   |           |   +---------+    |   +---+          |
 * |   |      |    | = |         |    | - |   |           | - |      (r1,c2) | + |   (r1,c1)    |
 * |   |      |    |   |         |    |   |   |           |   |              |   |              |
 * |   +------+    |   +---------+    |   +---+           |   |              |   |              |
 * |        (r2,c2)|   |       (r2,c2)|   |   (r2,c1)     |   |              |   |              |
 * +---------------+   +--------------+   +---------------+   +--------------+   +--------------+ */

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        use std::cmp::{max, min};
        let (m, n) = (mat.len(), mat[0].len());
        let mut range_sum = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                range_sum[i + 1][j + 1] =
                    range_sum[i + 1][j] + range_sum[i][j + 1] - range_sum[i][j] + mat[i][j];
            }
        }

        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let (r1, c1, r2, c2) = (
                    max(0, i as i32 - k) as usize,
                    max(0, j as i32 - k) as usize,
                    min(m as i32, i as i32 + k + 1) as usize,
                    min(n as i32, j as i32 + k + 1) as usize,
                );
                ans[i][j] =
                    range_sum[r2][c2] - range_sum[r2][c1] - range_sum[r1][c2] + range_sum[r1][c1];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1314() {}
}
