#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(nm) O(1)
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // What should we return when needle is an empty string? This is a great question
        // to ask during an interview.
        //
        //For the purpose of this problem, we will return 0 when needle is an empty string.
        //This is consistent to C's strstr() and Java's indexOf().
        if haystack.len() == 0 && needle.len() != 0 {
            return -1;
        }
        if haystack.len() == 0 || needle.len() == 0 {
            return 0;
        }

        let needle = needle.as_bytes();
        match haystack
            .as_bytes()
            .windows(needle.len())
            .position(|slice| slice == needle)
        {
            Some(x) => x as i32,
            _ => -1i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string(),), 2);
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string(),),
            -1
        );
        assert_eq!(Solution::str_str("".to_string(), "".to_string(),), 0);
    }
}
