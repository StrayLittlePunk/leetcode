#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n) O(1)
    pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];

        let mut start = 0;
        let mut end = nums.len() - 1;
        let get_sum = |num, a, b, c| a * num * num + b * num + c;

        let mut idx: i32;
        if a >= 0 {
            idx = (nums.len() - 1) as i32;
        } else {
            idx = 0;
        }

        while start <= end {
            let start_num = get_sum(nums[start], a, b, c);
            let end_num = get_sum(nums[end], a, b, c);

            if a >= 0 {
                if start_num >= end_num {
                    ans[idx as usize] = start_num;
                    idx -= 1;
                    start += 1;
                } else {
                    ans[idx as usize] = end_num;
                    idx -= 1;
                    end -= 1;
                }
            } else {
                if start_num <= end_num {
                    ans[idx as usize] = start_num;
                    idx += 1;
                    start += 1;
                } else {
                    ans[idx as usize] = end_num;
                    idx += 1;
                    end -= 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_360() {
        assert_eq!(
            Solution::sort_transformed_array(vec![-4, -2, 2, 4], 0, 3, 5),
            vec![-7, -1, 11, 17]
        );
        assert_eq!(
            Solution::sort_transformed_array(vec![-4, -2, 2, 4], 1, 3, 5),
            vec![3, 9, 15, 33]
        );
        assert_eq!(
            Solution::sort_transformed_array(vec![-4, -2, 2, 4], -1, 3, 5),
            vec![-23, -5, 1, 7]
        );
    }
}
