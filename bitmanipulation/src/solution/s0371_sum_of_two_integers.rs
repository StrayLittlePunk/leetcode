#![allow(unused)]
pub struct Solution {}

impl Solution {
    // string O(1) O(1)
    pub fn get_sum_string(a: i32, b: i32) -> i32 {
        let (mut x, mut y) = (i32::abs(a), i32::abs(b));
        // ensure that abs(a) >= abs(b)
        if x < y {
            return Self::get_sum(b, a);
        }

        // abs(A) >= abs(b) --> a determines the sign
        let mut sign = 1;
        if a < 0 {
            sign = -1;
        }

        if a * b >= 0 {
            // sum of two positive integers x + y
            // where x > y
            let (mut answer, mut carry) = (0, 0);
            while y != 0 {
                answer = x ^ y;
                carry = (x & y) << 1;
                x = answer;
                y = carry;
            }
        } else {
            // difference of two integers x - y
            // where x > y
            let (mut answer, mut borrow) = (0, 0);
            while y != 0 {
                answer = x ^ y;
                borrow = ((!x) & y) << 1;
                x = answer;
                y = borrow;
            }
        }

        x * sign
    }
    // O(1)  O(1)
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let (mut answer, mut carry) = (0, 0);
        while b != 0 {
            answer = a ^ b;
            carry = (a & b) << 1;
            a = answer;
            b = carry;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_371() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(-2, 3), 1);
    }
}
