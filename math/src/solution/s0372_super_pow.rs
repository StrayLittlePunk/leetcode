#![allow(unused)]
pub struct Solution {}

const BASE: i32 = 1337;
impl Solution {
    // O(n) O(1) n is length of vector
    pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
        let a = a % 1337;
        let mut b = b;
        let mut ret = 1;

        let tmp = match b.pop() {
            Some(n) => {
                for _ in 0..n {
                    ret *= a;
                    ret %= 1337;
                }
                Self::super_pow(a, b)
            }
            None => 1,
        };

        for _ in 0..10 {
            ret *= tmp;
            ret %= 1337;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_372() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
        assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
    }
}
