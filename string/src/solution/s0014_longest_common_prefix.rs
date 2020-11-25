#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

      let mut strs = strs.iter();

        if let Some(head) = strs.next().cloned() {
            strs.fold(head, |head, tail| {
                head
                    .chars()
                    .zip(tail.chars())
                    .take_while(|(l, r)| l == r)
                    .map(|(chr, _)| chr)
                    .collect()
            })
        } else {
            "".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string(),
            ]),
            "".to_string()
        );
    }
}
