#![allow(unused)]
use rand::{rngs::ThreadRng, thread_rng, Rng};
pub struct Solution {
    original: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            original: nums.clone(),
        }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut ans = self.original.clone();
        let len = ans.len();
        let mut rng = thread_rng();
        for i in 0..len {
            self.swap_at(&mut ans, i, rng.gen_range(i, len));
        }

        ans
    }

    fn swap_at(&self, ans: &mut Vec<i32>, a: usize, b: usize) {
        let tmp = ans[a];
        ans[a] = ans[b];
        ans[b] = tmp;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_384() {
        let obj = Solution::new(vec![1, 2, 3]);
        // assert_eq!(obj.shuffle(), vec![3, 1, 2]);
        assert_eq!(obj.reset(), vec![1, 2, 3]);
    }
}
