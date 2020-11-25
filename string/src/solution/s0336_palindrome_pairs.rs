#![allow(unused)]
pub struct Solution {}

// use std::collections::HashMap;
// impl Solution {
//    O(k^2 * n) O((k+n)^2)
//    n be the number of words, and k be the length of the longest word.
// fn all_valid_prefixes(word: &str) -> Vec<&str> {
// let mut ret = vec![];
// for i in 0..word.len() {
// if Self::is_palindrome(word, i as i32, word.len() as i32 - 1) {
// ret.push(&word[0..=i]);
// }
// }
//
// ret
// }
//
// fn all_valid_suffixes(word: &str) -> Vec<&str> {
// let mut ret = vec![];
// for i in 0..word.len() {
// if Self::is_palindrome(word, 0, i as i32) {
// ret.push(&word[i + 1..word.len()]);
// }
// }
//
// ret
// }
//
// fn is_palindrome(word: &str, mut left: i32, mut right: i32) -> bool {
// let ret = word.chars().collect::<Vec<char>>();
// while left < right && (right as usize) < ret.len() {
// if ret[left as usize] != ret[right as usize] {
// return false;
// }
// left += 1;
// right -= 1;
// }
// true
// }
//
// pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
// let (mut ans, mut word_set) = (vec![], HashMap::new());
//
// for i in 0..words.len() {
// word_set.insert(words[i].as_str(), i);
// }
//
// for word in words.iter() {
// let cur_idx = *word_set.get(&word.as_str()).unwrap();
// let rev_word = word.chars().rev().collect::<String>();
//
// if word_set.contains_key(&rev_word.as_str())
// && *word_set.get(&rev_word.as_str()).unwrap() != cur_idx
// {
// ans.push(vec![
// cur_idx as i32,
// *word_set.get(&rev_word.as_str()).unwrap() as i32,
// ]);
// }
//
// for suffix in Self::all_valid_suffixes(word.as_str()) {
// println!("suff {}", suffix);
// let rev_suffix = suffix.chars().rev().collect::<String>();
//
// if word_set.contains_key(&rev_suffix.as_str()) {
// println!("ans suff {}", cur_idx);
// ans.push(vec![
// *word_set.get(&rev_suffix.as_str()).unwrap() as i32,
// cur_idx as i32,
// ]);
// }
// }
//
// println!("word {}", word);
// for prefix in Self::all_valid_prefixes(word.as_str()) {
// println!("prefix {}", prefix);
// let rev_prefix = prefix.chars().rev().collect::<String>();
// if word_set.contains_key(&rev_prefix.as_str()) {
// ans.push(vec![
// cur_idx as i32,
// *word_set.get(&rev_prefix.as_str()).unwrap() as i32,
// ]);
// }
// }
// }
//
// ans
// }
// }

fn mk_idx(v: u8) -> usize {
    (v - b'a') as usize
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        // Need this ugly initialization because [x; N] require x to be copyable :(
        // Mapping from last letter to indices in `words`
        let mut last_let_to_indices = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        // Flag for possible empty string
        let mut empty_idx = None;
        for (i, w) in words.iter().map(String::as_bytes).enumerate() {
            match w.last() {
                Some(&c) => last_let_to_indices[mk_idx(c)].push(i),
                None => empty_idx = Some(i),
            }
        }

        let mut results = Vec::new();
        // Improved O(n^2)
        for (i, w1) in words.iter().map(String::as_bytes).enumerate() {
            // We will handle it later
            if w1.is_empty() {
                continue;
            }
            let first = mk_idx(*w1.first().unwrap());
            for &j in last_let_to_indices[first].iter() {
                if i == j {
                    continue;
                }
                let w2 = words[j].as_bytes();
                let sum_len_half = (w1.len() + w2.len()) / 2;
                // Rust iterators power allows to avoid extra allocation here
                let iter = w1.iter().copied().chain(w2.iter().copied());
                let reversed = iter.clone().rev();
                for (idx, (c1, c2)) in iter.zip(reversed).enumerate() {
                    if c1 != c2 {
                        break;
                    }
                    if idx >= sum_len_half {
                        results.push(vec![i as i32, j as i32]);
                        break;
                    }
                }
            }
        }

        // Case when one of words is empty
        // O(n)
        if let Some(empty_idx) = empty_idx {
            for (i, w) in words.iter().map(String::as_bytes).enumerate() {
                if i == empty_idx {
                    continue;
                }
                let len_half = w.len() / 2;
                let iter = w.iter().copied();
                let reversed = iter.clone().rev();
                for (idx, (c1, c2)) in iter.zip(reversed).enumerate() {
                    if c1 != c2 {
                        break;
                    }
                    if idx >= len_half {
                        results.push(vec![i as i32, empty_idx as i32]);
                        results.push(vec![empty_idx as i32, i as i32]);
                        break;
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_336() {
        assert_eq!(
            Solution::palindrome_pairs(vec![
                "a".to_owned(),
                "abc".to_owned(),
                "aba".to_owned(),
                "".to_owned()
            ]),
            vec![vec![0, 3], vec![3, 0], vec![2, 3], vec![3, 2]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec![
                "abcd".to_owned(),
                "dcba".to_owned(),
                "lls".to_owned(),
                "s".to_owned(),
                "sssll".to_owned()
            ]),
            vec![vec![0, 1], vec![1, 0], vec![2, 4], vec![3, 2]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec!["bat".to_owned(), "tab".to_owned(), "cat".to_owned()]),
            vec![vec![0, 1], vec![1, 0]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec!["a".to_owned(), "".to_owned()]),
            vec![vec![0, 1], vec![1, 0]]
        );
    }
}
