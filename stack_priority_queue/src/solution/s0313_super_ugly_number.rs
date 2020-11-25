#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
use std::i32;

impl Solution {
    // DP Time O(Nk) Space O(N) N is input , k is size of primes
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        if n <= 0 {
            return 1;
        }
        let mut nums: Vec<i32> = Vec::with_capacity(n as usize);
        let mut idx: Vec<usize> = vec![0;primes.len()];

        nums.push(1);
        for i in 1..n {
            let mut ugly = i32::MAX;
            for k in 0..primes.len() {
              ugly= min(ugly, nums[idx[k]] * primes[k]);
            }

            nums.push(ugly);
            for k in 0..idx.len() {
              if ugly == nums[idx[k]] * primes[k]{
                idx[k] += 1;
              }
            }
        }

        nums[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_313() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    }
}
