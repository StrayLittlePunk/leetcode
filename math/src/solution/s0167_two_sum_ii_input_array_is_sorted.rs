#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n) O(1)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut lptr, mut rptr) = (0, nums.len() - 1);
        let mut ans = vec![];

        while lptr != rptr {
            let sum = nums[lptr] + nums[rptr];
            if sum == target {
                ans.push(lptr as i32 + 1);
                ans.push(rptr as i32 + 1);
                return ans;
            } else if sum < target {
                lptr += 1;
            } else {
                rptr -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), [1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), [1, 2]);
    }
}
