#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(n)
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                stack.push(ch);
            } else {
                if let Some(c) = stack.pop() {
                    if (ch == ')' && c == '(') || (ch == '}' && c == '{') || (ch == ']' && c == '[')
                    {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()".to_owned()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_owned()), true);
        assert_eq!(Solution::is_valid("{()}".to_owned()), true);
        assert_eq!(Solution::is_valid("([)]".to_owned()), false);
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
    }
}
