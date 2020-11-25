#![allow(unused)]
pub struct Solution {}

impl Solution {
   // O(log n) space O(1)
    pub fn is_perfect_square_builtin(num: i32) -> bool {
      let root = (num as f32).sqrt() as i32;
      root * root == num
    }
   // O(log n) space O(1)
    pub fn is_perfect_square(num: i32) -> bool {
        if num <= 1 {
            return true;
        }
        let mut ans = num / 2;
        loop {
            let temp = (ans + num / ans) / 2;
            if temp >= ans {
                break;
            }
            ans = temp;
        }

        ans * ans == num

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_367() {
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(14), false);
    }
}
