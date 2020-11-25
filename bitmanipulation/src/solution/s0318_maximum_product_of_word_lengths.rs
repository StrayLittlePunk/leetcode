#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // Complexity Analysis
    // Time complexity : O(N^2 + L)
    // where N is a number of words and L is a total length of all words together.
    // The precomputation takes O(L) time because we iterate over all characters in all words.
    // The runtime word comparison takes O(N^2)  time. In total, that results in O(N^2 + L)
    // Space complexity : O(N) to keep two arrays of N elements
    //
    // 
    pub fn max_product(words: Vec<String>) -> i32 {
        if words.is_empty() {
            return 0;
        }

        let n = words.len();
        let mut masks = vec![0; n];
        let bit_number = |ch| ch as u32 - 'a' as u32;

        //Bitmasks + Precomputation : Comparison in O(1) time

        for i in 0..n {
            let mut bitmask = 0;
            for ch in words[i].chars() {
                // add bit number bit_number in bitmask
                bitmask |= 1 << bit_number(ch);
            }
            masks[i] = bitmask;
        }

        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                if masks[i] & masks[j] == 0 {
                    ans = max(ans, words[i].len() * words[j].len());
                }
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_318() {
        assert_eq!(
            Solution::max_product(vec![
                "abcw".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "bar".to_string(),
                "xtfn".to_string(),
                "abcdef".to_string(),
            ]),
            16
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_string(),
                "ab".to_string(),
                "abc".to_string(),
                "d".to_string(),
                "cd".to_string(),
                "bcd".to_string(),
                "abcd".to_string(),
            ]),
            4
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string(),
            ]),
            0
        );
    }
}
