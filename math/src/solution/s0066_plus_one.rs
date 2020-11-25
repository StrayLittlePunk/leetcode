#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(N + N), space O(1)
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut n = (digits.len() - 1) as i32;

        while n >= 0 {
            // add one with carry
            if digits[n as usize] == 9 {
                digits[n as usize] = 0;
                n -= 1;
            } else {
                digits[n as usize] = digits[n as usize] + 1;
                return digits;
            }
        }

        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
    }
}
