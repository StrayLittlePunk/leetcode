#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(log n) ~ O(n) O(1)
    pub fn integer_replacement(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        
        let mut n = n as u32;
        let mut count = 0;

        while n != 1 {
            if (n & 1) == 0 {
                n >>= 1;
            } else if n == 3 || ((n >> 1) & 1) == 0 {
                n -= 1;
            } else {
                n += 1;
            }
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_397() {
        assert_eq!(Solution::integer_replacement(8), 3);
        assert_eq!(Solution::integer_replacement(7), 4);
    }
}
