#![allow(unused)]
pub struct Solution {}

// amazon interview
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        use std::collections::HashMap;
        let (mut stack, chs, mut ret, mut map) = (vec![], chars.chars().collect::<Vec<char>>(), 0, HashMap::new());

        // generate hashmap to (char -> count)
        for &ch in chs.iter() {
            *map.entry(ch).or_insert(0) += 1;
        }

        for s in words.iter() {
            for ch in s.chars() {
              // not contains key
              if !map.contains_key(&ch) || *map.get(&ch).unwrap() <= 0{
                  break;
              }
             stack.push(ch);
             *map.entry(ch).or_insert(0) -= 1;

            }

            // can be formed by characters from chars
            if stack.len() == s.len() {
                ret += s.len();
            }
            // pop all char abjust all orginal state
            while let Some(ch) = stack.pop() {
                *map.entry(ch).or_insert(0) += 1;
            }
        }

        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1160() {
        
    }
}
