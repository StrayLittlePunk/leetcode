#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
use std::i32::MIN;

impl Solution {
    // O(n^2) O(1)
    // Time Limit Exceeded
    pub fn max_rotate_function_burte_force(a: Vec<i32>) -> i32 {
        if a.len() < 1 {
            return 0;
        }
        let mut ans = MIN;
        let mut idx = a.len();
        let mut sum = 0;
        for i in 0..a.len() {
            sum = 0;
            for j in 0..a.len() {
                sum += j as i32 * a[(idx + j - i) % a.len()];
            }

            ans = max(ans, sum);
        }

        ans
    }
    //     f(i)          = 0 * A[0] + 1 * A[1] + 2 * A[2] + .... +  (k-1) * A[k-1] + k * A[k]
    //     f(i+1)        = 1 * A[0] + 2 * A[1] + 3 * A[2] + ...  +     k  * A[k-1] + 0 * A[k]
    //  => f(i+1) - f(i) =     A[0]   +   A[1]   +   A[2] + ...  +       A[k-1]    - k * A[k]
    //     = (A[0]   +   A[1]   +   A[2] + ...  +       A[k-1] + k * A[k]) - (k+1) * A[k]
    //     = sum(Array) - A[k] * array.length
    //  => f(i+1) = f(i) + sum(Array) -  last element * array.length
    //    What is last element ?
    //     k = 1; last element = A[len-1];
    //     k = 2; last element = A[len-2];
    //     ...
    //
    //  O(n) O(1)
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        if a.len() < 1 {
            return 0;
        }

        let mut fi = 0;
        let mut array_sum = 0;

        for i in 0..a.len() {
            fi += i as i32 * a[i];
            array_sum += a[i];
        }

        let mut ans = fi;

        for k in (1..a.len()).rev() {
            fi = fi + array_sum - a.len() as i32 * a[k];

            ans = max(ans, fi);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_396() {
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
    }
}
