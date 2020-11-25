#![allow(unused)]
pub struct Solution {}

use std::collections::{BinaryHeap, VecDeque};
impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
        if k == 0 {
            return s;
        }

        let mut freq = [0; 26];

        let mut ans = "".to_string();

        for c in s.chars() {
            freq[(c as u32 - 'a' as u32) as usize] += 1;
        }

        let mut heap = BinaryHeap::new();
        for i in 0..26 {
            if freq[i] > 0 {
                heap.push((freq[i], i));
            }
        }

        let mut queue = VecDeque::new();
        while let Some((elem, idx)) = heap.pop() {
            ans.push(std::char::from_u32(idx as u32 + 'a' as u32).unwrap());

            queue.push_back((elem - 1, idx));
            if queue.len() >= k as usize {
                if let Some((elem, idx)) = queue.pop_front() {
                    if elem > 0 {
                        heap.push((elem, idx));
                    }
                }
            }
        }

        if ans.len() == s.len() {
            return ans;
        } else {
            return "".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_358() {
        assert_eq!(
            Solution::rearrange_string("aabbcc".to_string(), 3),
            "cbacba".to_string()
        );
        assert_eq!(
            Solution::rearrange_string("aaabc".to_string(), 3),
            "".to_string()
        );
    }
}
