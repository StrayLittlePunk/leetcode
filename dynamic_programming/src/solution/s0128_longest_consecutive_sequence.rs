#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
use std::cmp::max;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

      let mut set = HashSet::new();
      for &num in nums.iter() {
        set.insert(num);
      }

      let mut longest_streak = 0;
      for &num in set.iter() {

        // 要枚举的数 x 一定是在数组中不存在前驱数 x-1 的，不然按照上面的分析我们会从 x-1
        // 开始尝试匹配，因此我们每次在哈希表中检查是否存在 x-1即能判断是否需要跳过了。
        if !set.contains(&(num - 1)) {
          let mut cur_num = num;
          let mut cur_streak = 1;

          while set.contains(&(cur_num +1)) {
            cur_num += 1;
            cur_streak += 1;
          }

          longest_streak = max(longest_streak, cur_streak);
        }
      }
    
      longest_streak

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
