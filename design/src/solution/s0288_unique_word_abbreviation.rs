#![allow(unused)]
use std::collections::{HashMap, HashSet};

struct ValidWordAbbr {
    abbr_dict: HashMap<String, bool>,
    dict: HashSet<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut abbr_dict = HashMap::new();
        for s in dictionary.iter() {
            let abbr = Self::to_abbr(s);
            let b = abbr_dict.contains_key(&abbr);
            abbr_dict.insert(abbr, !b);
        }

        let dict = dictionary.into_iter().collect::<HashSet<String>>();
        Self {
            abbr_dict: abbr_dict,
            dict: dict,
        }
    }

    fn to_abbr(s: &String) -> String {
        let mut ret = String::from("");

        let n = s.len();
        if n <= 2 {
            return s.clone();
        }

        ret.push(s.chars().nth(0).unwrap());
        ret.push_str((n - 2).to_string().as_str());
        ret.push(s.chars().nth(n - 1).unwrap());

        ret
    }

    fn is_unique(&self, word: String) -> bool {
        let abbr = Self::to_abbr(&word);
        match self.abbr_dict.get(&abbr) {
            None => true,
            Some(&a) => a && self.dict.contains(&word),
        }
    }
}

/**
 * Your ValidWordAbbr object will be instantiated and called as such:
 * let obj = ValidWordAbbr::new(dictionary);
 * let ret_1: bool = obj.is_unique(word);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_288() {}
}
