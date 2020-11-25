#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    // Time Complexity: O(NKlogK), where N is the length of strs, and K is the maximum
    // length of a string in strs. The outer loop has complexity O(N) as we iterate
    // through each string. Then, we sort each string in O(KlogK) time.

    // Space Complexity: O(NK), the total information content stored in ans.
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // K: sorted(chars)  V : Vec<String>
        let mut ans = HashMap::new();
        for word in strs.iter() {
            let mut chs = word.chars().collect::<Vec<char>>();
            chs.sort();

            let val = ans.entry(chs).or_insert(vec![]);
            val.push(word.to_string());
        }

        ans.into_iter()
            .map(|(_, vals)| vals)
            .collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49() {
        // assert_eq!(
            // Solution::group_anagrams(vec![
                // "eat".to_string(),
                // "tea".to_string(),
                // "tan".to_string(),
                // "ate".to_string(),
                // "nat".to_string(),
                // "bat".to_string(),
            // ]),
            // vec![
                // vec!["bat".to_string(),],
                // vec!["nat".to_string(), "tan".to_string(),],
                // vec!["ate".to_string(), "eat".to_string(), "tea".to_string(),]
            // ]
        // );
        // assert_eq!(
            // Solution::group_anagrams(vec!["".to_string(),]),
            // vec![vec!["".to_string(),]]
        // );
        // assert_eq!(
            // Solution::group_anagrams(vec!["a".to_string(),]),
            // vec![vec!["a".to_string(),]]
        // );
    }
}
