#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(2^n) O(2^n)
    pub fn count_and_say(n: i32) -> String {

        if n < 2 {
            return "1".to_string();
        }

        let prev_res = Self::count_and_say(n - 1).chars().collect::<Vec<char>>();
        let mut ret = vec![];

        let mut prev_chr = &prev_res[0];
        let mut count = 1;

        for ch in &prev_res[1..] {
            if ch == prev_chr {
                count += 1;
            } else {
                ret.push(format!("{}{}", count, prev_chr));
                count = 1;
            }

            prev_chr = ch;
        }

        ret.push(format!("{}{}", count, prev_chr));
        ret.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1,), "1".to_string());
        assert_eq!(Solution::count_and_say(4,), "1211".to_string());
    }
}
