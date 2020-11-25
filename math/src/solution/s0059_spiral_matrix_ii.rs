#![allow(unused)]
pub struct Solution {}

use std::usize::MAX;
impl Solution {
    // O(n^2) O(1)
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n as usize]; n as usize];

        if n < 1 {
            return ans;
        }

        let (mut r1, mut r2, mut c1, mut c2, mut idx) = (0, ans.len() - 1, 0, ans[0].len() - 1, 1);

        while r1 <= r2 && c1 <= c2 && r2 != MAX && c2 != MAX {
            // top loop
            for c in c1..=c2 {
                ans[r1][c] = idx;
                idx += 1;
            }

            for r in r1 + 1..=r2 {
                ans[r][c2] = idx;
                idx += 1;
            }

            if r1 < r2 && c1 < c2 {
                for c in (c1 + 1..c2).rev() {
                    ans[r2][c] = idx;
                    idx += 1;
                }

                for r in (r1 + 1..=r2).rev() {
                    ans[r][c1] = idx;
                    idx += 1;
                }
            }
            r1 += 1;
            r2 -= 1;
            c1 += 1;
            c2 -= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}
