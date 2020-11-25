#![allow(unused)]
pub struct Solution {}

impl Solution {
    /*
        荷兰三色旗问题解
    */
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // 对于所有 idx < i : nums[idx < i] = 0
        // j是当前考虑元素的下标
        // 对于所有 idx > k : nums[idx > k] = 2
        let mut p0: i32 = 0;
        let mut p2: i32 = (nums.len() - 1) as i32;

        let mut curr: i32 = 0;

        while curr <= p2 {
            if nums[curr as usize] == 0 {
                // 交换第 p0个和第curr个元素
                // i++，j++
                let tmp = nums[p0 as usize];
                nums[p0 as usize] = nums[curr as usize];
                nums[curr as usize] = tmp;
                curr += 1;
                p0 += 1;
            } else if nums[curr as usize] == 2 {
                // 交换第k个和第curr个元素
                // p2--
                let tmp = nums[curr as usize];
                nums[curr as usize] = nums[p2 as usize];
                nums[p2 as usize] = tmp;
                p2 -= 1
            } else {
                curr += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_75() {
        let mut nums: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }
}
