#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();
        Solution::helper(&s, &word_dict, &mut hm)
    }
    fn helper(s: &str, words: &[String], hm: &mut HashMap<String, Vec<String>>) -> Vec<String> {
        if let Some(v) = hm.get(&s.to_string()) {
            return v.clone();
        }
        let mut ret = Vec::new();
        for word in words.iter() {
            if s.starts_with(word) {
                for e in Solution::helper(&s[word.len()..], words, hm).iter() {
                    let mut ss = word.clone();
                    ss.push(' ');
                    ret.push(ss + e);
                }
                if word == s {
                    ret.push(word.clone());
                }
            }
        }
        hm.insert(s.to_string(), ret.clone());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_140() {
        assert_eq!(
            Solution::word_break(
                "catsanddog".to_owned(),
                vec![
                    "cat".to_owned(),
                    "cats".to_owned(),
                    "and".to_owned(),
                    "sand".to_owned(),
                    "dog".to_owned(),
                ]
            ),
            vec![
            "cat sand dog".to_owned(),
            "cats and dog".to_owned(), 
            ]
        );
    }
}
