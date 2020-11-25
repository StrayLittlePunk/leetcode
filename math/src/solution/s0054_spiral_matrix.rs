#![allow(unused)]
pub struct Solution {}
use std::usize::MAX;
impl Solution {
    // O(N) N is total number of elements in the input matrix , O(1)
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        if matrix.len() < 1 {
            return ans;
        }

        let (mut r1, mut r2, mut c1, mut c2) = (0, matrix.len() - 1, 0, matrix[0].len() - 1);
        while r1 <= r2 && c1 <= c2 && r2 != MAX && c2 != MAX {
            // top loop
            for c in c1..=c2 {
                ans.push(matrix[r1][c]);
            }
            // right loop
            for r in r1 + 1..=r2 {
                ans.push(matrix[r][c2]);
            }
            if r1 < r2 && c1 < c2 {
                // bottom loop
                for c in (c1 + 1..c2).rev() {
                    ans.push(matrix[r2][c]);
                }
                // left loop
                for r in (r1 + 1..=r2).rev() {
                    ans.push(matrix[r][c1]);
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
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
