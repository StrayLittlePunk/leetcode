#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n^2) O(1)
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;

        if nums.len() < 3 {
            return ans;
        }

        nums.sort();

        //  [1, 2, 3, 5, 8]
        //   ↑        ↑
        //  left    right
        // How many pairs with one of the index = left that satisfy the condition?
        // You can tell by the difference between right and left which is 33,
        // namely (1,2), (1,3), and (1,5). Therefore, we move left one step to its right
        for i in 0..nums.len() {
            let (mut lptr, mut rptr) = (i + 1, nums.len() - 1);
            while lptr < rptr {
                if nums[i] + nums[lptr] + nums[rptr] < target {
                    ans += (rptr - lptr) as i32;
                    lptr += 1;
                } else {
                    rptr -= 1;
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
    fn test_259() {
        assert_eq!(Solution::three_sum_smaller(vec![-2, 0, 1, 3], 2), 2);
        assert_eq!(Solution::three_sum_smaller(vec![], 0), 0);
        assert_eq!(Solution::three_sum_smaller(vec![0], 0), 0);
    }
}
