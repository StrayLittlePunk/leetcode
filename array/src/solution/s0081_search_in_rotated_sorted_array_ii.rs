#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
       if nums.len() == 0 { return false; }
        let (mut l, mut mid, mut h) = (0, 0, nums.len() -1);
        while l < h {
          mid = (h + l) >> 1;
          if (nums[mid] == target) {
            return true;
          }
          else if (nums[mid] > nums[h]) {
            if nums[mid] > target && nums[l] <= target {
              h = mid;
            }else {
              l = mid + 1;
            }
          } else if nums[mid] < nums[h] {
            if nums[mid] <target && nums[h] >= target {
              l = mid + 1;
            }else {
              h = mid;
            }
          } else {
            h -= 1;
          }
        }

        nums[l] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
