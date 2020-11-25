#![allow(unused)]
pub struct Solution {}

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    //     Complexity Analysis
    //     Time O(M^2 x N) where M is the length of each word and N is the total number of words
    //     in the input word list.
    //     For each word in the word list, we iterate over its length to find all the intermediate
    //     words corresponding to it. Since the length of each word is M and we have N words,
    //     the total number of iterations the algorithm takes to create all_combo_dict is M × N.
    //     Additionally, forming each of the intermediate word takes O(M) time because of
    //     the substring operation used to create the new string. This adds up to a complexity of
    //     O(M^2 * N)

    //  Breadth first search in the worst case might go to each of the N words. For each word,
    //  we need to examine M possible intermediate words/combinations. Notice, we have used
    //  the substring operation to find each of the combination. Thus, M combinations take
    //  O(M^ 2)  time. As a result, the time complexity of BFS traversal would also
    //  be O(M^2 x N)
    //
    // Space Complexity: O(M^2 x N)
    // same as above
    //
    // Optimization: We can definitely reduce the space complexity of this algorithm by storing
    // the indices corresponding to each word instead of storing the word itself.
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }
        // Since all words are of same length.
        let len = begin_word.len();

        // Dictionary to hold combination of words that can be formed,
        // from any given word. By changing one letter at a time.
        let mut allcombo_dict = HashMap::new();

        for word in word_list.iter() {
            for i in 0..len {
                // Key is the generic word
                // Value is a list of words which have the same intermediate generic word.
                // create a string need O(M) time
                let new_word = format!("{}{}{}", &word[0..i], "*", &word[i + 1..len]);
                let transformations = allcombo_dict.entry(new_word).or_insert(vec![]);
                transformations.push(word.to_string());
            }
        }

        // Visited to make sure we don't repeat processing same word.
        let mut visited = HashSet::new();
        visited.insert(begin_word.to_string());

        // queue for BFS
        let mut queue = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((word, level)) = queue.pop_front() {
            for i in 0..len {
                // Intermediate words for current word
                let intermediate_word = format!("{}{}{}", &word[0..i], "*", &word[i + 1..len]);
                // Next states are all the words which share the same intermediate state.
                if let Some(trans) = allcombo_dict.get(&intermediate_word) {
                    // If at any point if we find what we are looking for
                    // i.e. the end word - we can return with the answer.
                    for next in trans.iter() {
                        // Compare a string need O(M) time
                        if *next == end_word {
                            return level + 1;
                        }
                        // otherwise, add it to the BFS Queue. Also mark it visited
                        if !visited.contains(next) {
                            visited.insert(next.to_string());
                            queue.push_back((next.to_string(), level + 1));
                        }
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_127() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string(),
                ]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                ]
            ),
            0
        );
    }
}
