#![allow(unused)]
pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    // O(n) O(n)
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(map: &mut HashMap<i32, i32>, nums: &Vec<i32>, target: i32) -> i32 {
            if target == 0 {
                return 1;
            } else if target < 0 {
                return 0;
            }

            let mut sum = 0;

            for &num in nums.iter() {
                if map.contains_key(&(target - num)) {
                    sum += *map.get(&(target - num)).unwrap();
                } else {
                    let ret = helper(map, nums, target - num);
                    map.insert(target - num, ret);
                    sum += ret;
                }
            }
            return sum;
        }

        // DP
        let mut map = HashMap::new();

        helper(&mut map, &nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_377() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }
}
