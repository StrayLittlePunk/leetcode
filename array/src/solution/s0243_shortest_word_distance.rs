#![allow(unused)]
pub struct Solution {}

// uber interview

use std::cmp::min;
impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut distance, mut w1, mut w2) = (
            words.len() as i32,
            words.len() as i32,
            2 * words.len() as i32,
        );
        for i in 0..words.len() {
            if word1.as_str() == words[i].as_str() {
                w1 = i as i32;
            }

            if word2.as_str() == words[i].as_str() {
                w2 = i as i32;
            }

            distance = min(distance, i32::abs(w1 - w2));
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1053() {}
}
