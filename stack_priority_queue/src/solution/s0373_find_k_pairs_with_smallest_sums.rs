#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Min Heap Time O(k log k) Space O(k) 
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        use std::cmp::min;
        use std::collections::BinaryHeap;
        let mut ans = vec![];
        if k == 0 || nums1.is_empty() || nums2.is_empty() {
            return ans;
        }

        let mut heap = BinaryHeap::<Vec<i32>>::new();
        for i in 0..min(k as usize, nums1.len()) {
            heap.push(vec![-(nums1[i] + nums2[0]), nums1[i], nums2[0], 0]);
        }
        while k > 0 && !heap.is_empty() {
            k -= 1;
            let cur = heap.pop().unwrap();
            ans.push(vec![cur[1], cur[2]]);
            if cur[3] == nums2.len() as i32 - 1 {
                continue;
            }
            heap.push(vec![
                -cur[1] - nums2[cur[3] as usize + 1],
                cur[1],
                nums2[cur[3] as usize + 1],
                cur[3] + 1,
            ]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_373() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );
    }
}
