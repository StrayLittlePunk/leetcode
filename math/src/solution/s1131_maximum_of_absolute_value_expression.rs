#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
impl Solution {
    /*     Intuition
     * Take |x[i] - x[j]| + |y[i] - y[j]| as Manhattan distance of two points.
     * x is the coordinate of points on the x-axis,
     * y is the coordinate of points on the y-axis.
     *
     *
     * Explanation 1: Math
     * Assume i < j, there are four possible expression:
     * |x[i] - x[j]| + |y[i] - y[j]| = (x[i] - x[j]|) + (y[i] - y[j]) = (x[i] + y[i]|) - (x[j] + y[j])
     * |x[i] - x[j]| + |y[i] - y[j]| = (x[i] - x[j]|) - (y[i] - y[j]) = (x[i] - y[i]|) - (x[j] - y[j])
     * |x[i] - x[j]| + |y[i] - y[j]| = -(x[i] - x[j]|) + (y[i] - y[j]) = (-x[i] + y[i]|) - (-x[j] + y[j])
     * |x[i] - x[j]| + |y[i] - y[j]| = -(x[i] - x[j]|) - (y[i] - y[j]) = (-x[i] - y[i]|) - (-x[j] - y[j])
     *
     * So we can see, the expression
     * |x[i] - x[j]| + |y[i] - y[j]| + |i - j| = f(j) - f(i)
     *
     * where f(i) = p * x[i] + q * y[i] + i
     * with p = 1 or -1, q = 1 or -1 */
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let (mut ret, n) = (0, arr1.len());
        for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)].iter() {
            let mut min_val = p * arr1[0] + q * arr2[0] + 0;
            for i in 0..n {
                let cur = p * arr1[i] + q * arr2[i] + i as i32;
                ret = max(ret, cur - min_val);
                min_val = min(min_val, cur);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1331() {}
}
