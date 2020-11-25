#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut aux: Vec<i32> = nums.iter().map(|&v| v).collect();
        let mut sz = 1;
        loop {
            if !(sz < n) {
                break;
            }
            let mut lo = 0;
            loop {
                if !(lo < n - sz) {
                    break;
                }
                Self::merge(
                    &mut nums,
                    &mut aux,
                    lo,
                    lo + sz - 1,
                    min(lo + sz + sz - 1, n - 1),
                );
                lo += sz + sz;
            }
            sz = sz + sz;
        }

        return nums;
    }

    fn merge(a: &mut Vec<i32>, aux: &mut Vec<i32>, lo: usize, mid: usize, hi: usize) {
        for k in lo..hi + 1 {
            aux[k] = a[k];
        }

        let (mut i, mut j) = (lo, mid + 1);
        for k in lo..hi + 1 {
            if i > mid {
                a[k] = aux[j];
                j += 1;
            } else if j > hi {
                a[k] = aux[i];
                i += 1;
            } else if aux[j] < aux[i] {
                a[k] = aux[j];
                j += 1;
            } else {
                a[k] = aux[i];
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), [1, 2, 3, 5]);
    }
}
