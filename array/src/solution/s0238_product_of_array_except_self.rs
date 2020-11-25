#![allow(unused)]

pub struct Solution {}

// https://leetcode.com/problems/product-of-array-except-self/description/

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return Vec::new();
        }
        // answer[i] 表示索引 i 左侧所有元素的乘积
        // 因为索引为 '0' 的元素左侧没有元素， 所以 answer[0] = 1 
        let mut ans = vec!(1;nums.len());
        let mut r = 1;
        for i in 1..nums.len() {
          ans[i] = ans[i - 1] * nums[i -1];
        }

        // r 为右侧所有元素的乘积
        // 刚开始右边没有元素，所以 r = 1
        let mut i = nums.len() - 1;
        for i in (0..nums.len()).rev() {
          // 对于索引 i，左边的乘积为 answer[i]，右边的乘积为 r
          ans[i] = ans[i] *r;
          // r 需要包含右边所有的乘积，所以计算下一个结果时需要将当前值乘到 r 上
          r *= nums[i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_238() {
        assert_eq!(Solution::product_except_self( vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
