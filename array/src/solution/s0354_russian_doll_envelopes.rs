#![allow(unused)]
pub struct Solution {}

/* 354. Russian Doll Envelopes
 * Hard
 *
 * 1404
 *
 * 47
 *
 * Add to List
 *
 * Share
 * You have a number of envelopes with widths and heights given as a pair of integers (w, h). One envelope can fit into another if and only if both the width and height of one envelope is greater than the width and height of the other envelope.
 *
 * What is the maximum number of envelopes can you Russian doll? (put one inside other)
 *
 * Note:
 * Rotation is not allowed.
 *
 * Example:
 *
 * Input: [[5,4],[6,4],[6,7],[2,3]]
 * Output: 3
 * Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).
 *  */
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
