#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(N) O(N)
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let mut m1 = HashMap::<char, usize>::new();
        let mut m2 = HashMap::<char, usize>::new();
        for (i, (ch1, ch2)) in s.chars().zip(t.chars()).enumerate() {
            if m1.insert(ch1, i) != m2.insert(ch2, i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );



    }
}
