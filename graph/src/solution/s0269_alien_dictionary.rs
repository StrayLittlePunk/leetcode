#![allow(unused)]
pub struct Solution {}

use std::collections::{HashMap, VecDeque};
impl Solution {

  // Time O(C) C is the total length of all the words in the input list, added together.
  // Space O(1) or O(U + min(U^2, N))
    pub fn alien_order(words: Vec<String>) -> String {
        if words.len() < 1 {
            return "".to_string();
        }

        // Step 0: create data structures
        let mut words_map = HashMap::new();
        let mut in_degree = HashMap::new();

        for word in words.iter() {
          for c in word.chars() {
            in_degree.insert(c, 0);
          }
        }

        //  Step 1: We need to populate adj_list and in_degree.
        // For each pair of adjacent words...
        for (word1, word2) in words.iter().zip(words[1..].iter()) {
            // Check that word2 is not a prefix of word1.
            if word1.len() > word2.len() && word1.starts_with(word2) {
                return "".to_string();
            }
            // Find the first non match and insert the corresponding relation.
            for (c, d) in word1.chars().zip(word2.chars()) {
                if c != d {
                    let set = words_map.entry(c).or_insert(vec![]);
                    set.push(d);
                    let count = in_degree.entry(d).or_insert(0);
                    *count += 1;
                    break;
                }
            }
        }

        // step2 BFS
        // We need to repeatedly pick off nodes with an indegree of 0.
        let mut ans = vec![];

        let mut queue = VecDeque::new();
        for c in in_degree.keys() {
            if *in_degree.get(c).unwrap() == 0 {
                queue.push_back(*c);

            }
        }

        while let Some(ch) = queue.pop_front() {
            ans.push(ch);
            if let Some(adj_list) = words_map.get(&ch) {
                for next in adj_list {
                    if let Some(cnt) = in_degree.get_mut(&next) {
                        *cnt -= 1;
                        if *cnt == 0 {
                            queue.push_back(*next);
                        }
                    }
                }
            }
        }

        // If not all letters are in output, that means there was a cycle and so
        // no valid ordering. Return "" as per the problem description.
        if ans.len() < in_degree.len() {
            return "".to_string();
        }

        // Otherwise, convert the ordering we found into a string and return it.
        ans.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_269() {
        assert_eq!(
            Solution::alien_order(vec![
                "wrt".to_string(),
                "wrf".to_string(),
                "er".to_string(),
                "ett".to_string(),
                "rftt".to_string(),
            ]),
            "wertf".to_string()
        );

        assert_eq!(
            Solution::alien_order(vec!["z".to_string(), "x".to_string()]),
            "zx".to_string()
        );
        assert_eq!(
            Solution::alien_order(vec!["z".to_string(), "x".to_string(), "z".to_string()]),
            "".to_string()
        );
    }
}
