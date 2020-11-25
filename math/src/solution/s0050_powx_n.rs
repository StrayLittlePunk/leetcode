#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        Self::fast_pow(x, n)
    }
    fn fast_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let half = Self::fast_pow(x, n / 2);
        if n % 2 == 0 {
            return half * half;
        } else {
            return half * half * x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {}
}
