#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return Solution::intersect(nums2, nums1);
        }

        let mut m: HashMap<i32, i32> = HashMap::new();

        for num in &nums1 {
            if m.contains_key(num) {
                m.insert(*num, *(m.get(num).unwrap()) + 1);
            } else {
                m.insert(*num, 1);
            }
        }

        let mut res: Vec<i32> = Vec::new();
        for num in &nums2 {
            let cnt = if m.contains_key(num) {
                *(m.get(num).unwrap())
            } else {
                0
            };
            if cnt > 0 {
                res.push(*num);
                m.insert(*num, cnt - 1);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_350() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![9, 4]
        );
    }
}
