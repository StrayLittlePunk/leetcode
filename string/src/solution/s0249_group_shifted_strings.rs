#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
  // O(NM) N is the length of strings, M is length of string
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut ret = HashMap::new();

        for s in strings.into_iter() {
            let key = Self::get_key(s.as_str());
            let list = ret.entry(key).or_insert(vec![]);
            list.push(s);
        }

        ret.into_iter()
            .map(|(_, vals)| vals)
            .collect::<Vec<Vec<String>>>()
    }

    fn get_key(s: &str) -> String {
        let chs = s.chars().collect::<Vec<char>>();
        let mut key = String::from("");
        for i in 1..chs.len() {
            let diff = chs[i] as i32 - chs[i - 1] as i32;
            if diff < 0 {
                let ch = 'a' as u32 + (diff + 26) as u32;
                key.push(std::char::from_u32(ch).unwrap());
            } else {
                let ch = 'a' as u32 + diff as u32;
                key.push(std::char::from_u32(ch).unwrap());
            }
            key.push(',');
        }

        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_249() {
        // assert_eq!(
            // Solution::group_strings(vec![
                // "abc".to_string(),
                // "bcd".to_string(),
                // "acef".to_string(),
                // "xyz".to_string(),
                // "az".to_string(),
                // "ba".to_string(),
                // "a".to_string(),
                // "z".to_string(),
            // ],),
            // vec![
                // vec!["abc".to_string(), "bcd".to_string(), "xyz".to_string(),],
                // vec!["az".to_string(), "ba".to_string(),],
                // vec!["acef".to_string(),],
                // vec!["a".to_string(), "z".to_string(),]
            // ]
        // );
    }
}
