#![allow(unused)]
pub struct Solution {}

impl Solution {
   // O(log n) space O(1)
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        let mut ans = x / 2;
        loop {
            let temp = (ans + x / ans) / 2;
            if temp >= ans {
                break;
            }
            ans = temp;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
