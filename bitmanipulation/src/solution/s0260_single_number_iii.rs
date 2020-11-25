#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(N) Space O(1)
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // difference between two numbers (x and y) which were seen only once
        let mut bitmask = 0;
        for num in nums.iter() {
            bitmask ^= *num;
        }

        // rightmost 1-bit diff between x and y
        let diff = bitmask & (-bitmask);

        let mut x = 0;
        // bitmask which will contain only x
        for num in nums {
            if (num & diff) != 0 {
                x ^= num;
            }
        }

        vec![x, bitmask ^ x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_260() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
        assert_eq!(Solution::single_number(vec![0, 1]), vec![1, 0]);
    }
}
