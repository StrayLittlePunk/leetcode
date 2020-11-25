#![allow(unused)]
pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::collections::VecDeque;
impl Solution {
    //   Complexity Analysis
    // Time O(N), Space O(N)
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        if nested_list.len() < 1 {
            return 0;
        }

        let mut queue = VecDeque::from(nested_list);
        // previous sum
        let mut prev = 0;
        // total sum 
        let mut ans = 0;
        
        // ans = prev +.. + prev  + level_sum 
        //        ^          ^
        //        0         depth - 1
        while !queue.is_empty() {
            let size = queue.len();
            let mut level_sum = 0;
            for i in 0..size {
                match queue.pop_front() {
                    Some(NestedInteger::Int(c)) => {
                        level_sum += c;
                    }
                    Some(NestedInteger::List(vec)) => {
                        for next in vec {
                            queue.push_back(next);
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            // great idea 
            prev += level_sum;
            ans += prev;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_339() {
        assert_eq!(
            Solution::depth_sum_inverse(vec![
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1),]),
                NestedInteger::Int(2),
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1),])
            ]),
            8
        );

        assert_eq!(
            Solution::depth_sum_inverse(vec![
                NestedInteger::Int(1),
                NestedInteger::List(vec![
                    NestedInteger::Int(4),
                    NestedInteger::List(vec![NestedInteger::Int(6)])
                ])
            ]),
            17
        );
    }
}
