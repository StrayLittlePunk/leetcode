#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn generate_possible_next_moves(s: String) -> Vec<String> {
        let mut ans = vec![];
        let str_slice = s.as_str();
        for i in 1..s.len() {
            if &str_slice[i..i + 1] == "+" && &str_slice[i - 1..i] == "+" {
                let mut ret = s.clone();
                ret.replace_range(i-1..i+1, "--");
                ans.push(ret);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_293() {
        assert_eq!(
            Solution::generate_possible_next_moves("++++".to_string(),),
            vec!["--++".to_string(), "+--+".to_string(), "++--".to_string(),]
        );
    }
}
