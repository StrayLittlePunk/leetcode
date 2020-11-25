#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
use std::i32::MIN;
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return 0;
        }

        let (m, n, mut ret) = (matrix.len(), matrix[0].len(), MIN);

        for left in 0..n {
            let mut row_sum = vec![0; m];
            for right in left..n {
                for i in 0..m {
                    row_sum[i] += matrix[i][right];
                }

                ret = max(ret, Self::dpmax(&row_sum, k));
                if ret == k {
                    return k;
                }
            }
        }

        ret
    }

    fn dpmax(row_sum: &Vec<i32>, k: i32) -> i32 {
        let mut roll_sum = row_sum[0];
        let mut roll_max = roll_sum;

        for i in 1..row_sum.len() {
            if roll_sum > 0 {
                roll_sum += row_sum[i];
            } else {
                roll_sum = row_sum[i];
            }

            if roll_sum > roll_max {
                roll_max = roll_sum;
            }
        }

        if roll_max <= k {
            return roll_max;
        }

        let mut ret = MIN;
        for left in 0..row_sum.len() {
            let mut sum = 0;
            for right in left..row_sum.len() {
                sum += row_sum[right];
                if sum > ret && sum <= k {
                    ret = sum;
                }

                if ret == k {
                    return k;
                }
            }
        }

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_365() {
        assert_eq!(
            Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
            2
        );
    }
}
