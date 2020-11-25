#![allow(unused)]
pub struct Solution {}


// https://leetcode.com/problems/jump-game-ii/


impl Solution {
   pub fn jump(nums: Vec<i32>) -> i32 {
     use std::cmp::max;
     let mut end :i32  = 0;
     let mut max_pos :i32  = 0;
     let mut steps :i32  = 0;

     for i in 0..(nums.len() - 1) {
       max_pos = max(max_pos, nums[i] + (i as i32));
       if ((i as i32) == end){
         end = max_pos;
         steps += 1;
       }
     }
    return steps;
   } 
}



// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        assert_eq!(
            Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
