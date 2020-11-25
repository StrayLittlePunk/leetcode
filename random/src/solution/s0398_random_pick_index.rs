#![allow(unused)]
use rand::{rngs::ThreadRng, thread_rng, Rng};
struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        let mut total = 0;
        let mut rng = thread_rng();
        let mut res = -1;
        for i in 0..self.nums.len() {
            if self.nums[i] == target {
                // randomly select an int from 0 to the nums of target. If x equals 0,
                // set the res as the current index. The probability is always 1/nums for the
                // latest appeared number. For example, 1 for 1st num, 1/2 for 2nd num,
                // 1/3 for 3nd num (1/2 * 2/3 for each of the first 2 nums).
                total += 1;
                if rng.gen_range(0, total) == 0 {
                    res = i as i32;
                }
            }
        }

        res
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_398() {
        // let obj = Solution::new(vec![1, 2, 3, 3, 3]);
         // assert!(obj.pick(3) == 2 || obj.pick(3) == 3 || obj.pick(3) == 4);
    }
}
