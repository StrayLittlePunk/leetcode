#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
use std::i32;

impl Solution {
    // Time O(N^2), Space O(1) Brute Force Solution
    pub fn largest_rectangle_area_brute(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for i in 0..heights.len() {
            let mut min_height = i32::MAX;
            for j in i..heights.len() {
                min_height = min(min_height, heights[j]);
                max_area = max(max_area, min_height * (j - i + 1) as i32);
            }
        }
        max_area
    }
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
       let mut stack=vec![];
        let mut res =0;
        for i in 0..=heights.len(){
            let cur_height=if i==heights.len(){0}else { heights[i] };

            while !stack.is_empty()&&cur_height< heights[*stack.last().unwrap()] {
                let h=stack.pop().unwrap();
                if stack.is_empty(){
                    res=res.max(i as i32*heights[h])
                }else { res=res.max((i-*stack.last().unwrap()-1) as i32* heights[h]); }

            }
            stack.push(i)
        }
        res

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
