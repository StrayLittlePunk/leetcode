#![allow(unused)]
pub struct Solution {}

impl Solution {
   // O(1) O(1)
   // https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity
   // https://leetcode.com/problems/water-and-jug-problem/discuss/83715/Math-solution-Java-solution
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if x + y < z {
            return false;
        }

        if x == z || y == z || x + y == z {
            return true;
        }

        z % Self::gcd(x, y) == 0
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_365() {
        assert_eq!(Solution::can_measure_water(3, 5, 4), true);
        assert_eq!(Solution::can_measure_water(2, 6, 5), false);
    }
}
