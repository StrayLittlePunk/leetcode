#![allow(unused)]
pub struct Solution {}
// amazon interview
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut banset: HashSet<String> = HashSet::from_iter(banned);
        let mut freq = HashMap::new();
        let words = paragraph.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace());
        for word in words {
            let word = word.to_ascii_lowercase();
            if word == "" || banset.contains(&word) {
                continue;
            }
            *freq.entry(word).or_insert(0) += 1;
        }
        let mut most = "";
        let mut most_count = 0;
        for (k, v) in freq.iter() {
            if v > &most_count {
                most_count = *v;
                most = k
            }
        }
        most.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_819() {}
}
