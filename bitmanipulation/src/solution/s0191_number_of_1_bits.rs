#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn hamming_weight (mut n: u32) -> i32 {
      let mut count = 0;
      while n > 0 {
        if (n & 0x1) == 0x1 {
          count += 1;
        }

        n >>= 1;
      }
        
      count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_191() {
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000000001011), 3);
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000010000000), 1);
        assert_eq!(Solution::hamming_weight(0b11111111111111111111111111111101), 31);
    }
}
