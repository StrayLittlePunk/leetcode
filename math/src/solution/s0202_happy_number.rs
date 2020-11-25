#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(log n) O(1) Floyd's Cycle-Finding Algorithm
  // altertive way hashset 
    pub fn is_happy(n: i32) -> bool {
      let (mut slow, mut fast) = (n, Self::get_next(n));

      while fast != 1 && slow != fast {
        slow = Self::get_next(slow);
        fast = Self::get_next(Self::get_next(fast));
      }


      fast == 1


    }

    fn get_next(mut n: i32) -> i32 {
      let mut sum = 0;
      while n > 0 {

        let d = n % 10;
        n /= 10;
        sum +=  d * d;
      }

      sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(Solution::is_happy(19), true);
    }
}
