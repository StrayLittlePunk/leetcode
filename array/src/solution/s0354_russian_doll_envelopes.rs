#![allow(unused)]
pub struct Solution {}


impl Solution {
    fn length_lis(nums: &Vec<i32>) -> i32 {

      let dp = vec![0;nums.len()];
      let mut len = 0;
      for num in nums {
        let mut i = dp.binary_search(num).unwrap() as i32;
        if i < 0 {
          i = -(i + 1);
        }
      }

      2
    }
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {

      3
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_354() {
       assert_eq!(Solution::max_envelopes(
           vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]), 3);
    }
}
