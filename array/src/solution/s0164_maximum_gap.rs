#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/maximum-gap/description/

use std::cmp::max;
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
      if nums.len() < 2 {
        return 0;
      }

      // *nums 解引用nums数组，等价于 *(nums.deref()) Rust 编译器先调用deref方法，
      // 返回自身的引用，然后进行进行普通的解引用，这一切都是编译期间发生的事，没有
      // 运行负担。第一次见到时有个疑惑，为什么不规定Deref的deref()方法返回真正的对象
      // 而要绕一步进行普通*解引用？原因在于Rust的所有权系统，如果这里的deref方法返回不是
      // 引用类型，那么该值的所有权随着转移出去，从而导致自身无法使用该数组，这也是Rust的
      // *操作符 是对deref返回的引用再常规解引用的原因。
      let max_v = *nums.iter().max().unwrap();
      let min_v = *nums.iter().min().unwrap();

      let bucket_size = max(1 as i32, (max_v - min_v) / nums.len() as i32);
      let bucket_count = (max_v - min_v) / bucket_size + 1;

      let mut buckets = vec![(None, None); bucket_count as usize];

      for n in &nums {
       let bucket_num = ((n - min_v) / bucket_size) as usize; 
       let bucket = buckets[bucket_num];

       match bucket {
         (Some(a), Some(b)) => {
           if n< a {
             buckets[bucket_num] = (Some(n), Some(b))
           } else if n > b {
             buckets[bucket_num] = (Some(a), Some(n))
           }
         }

         (None, None) => {
           buckets[bucket_num] = (Some(n), Some(n));
         }

         _ => ()
       }
      }
      let mut res = 0;
      let mut prev_max = min_v;
      for bucket in buckets {
        if let (Some(a), Some(b)) = bucket {
          res = max(res, a - prev_max);
          prev_max = *b;
        }
      }
    res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_164() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
