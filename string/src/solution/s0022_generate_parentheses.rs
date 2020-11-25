#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = vec![];
        Self::backtrack(&mut ans, "".to_owned(), 0, 0, n);
        ans
    }

    fn backtrack(ans: &mut Vec<String>, mut cur: String, open: i32, close: i32, max: i32) {
        if cur.len() == max as usize * 2 {
            ans.push(cur);
            return;
        }

        if open < max {
            let mut ret = cur.clone();
            ret.push('(');
            Self::backtrack(ans, ret, open + 1, close, max);
        }
        if close < open {
            cur.push(')');
            Self::backtrack(ans, cur, open, close + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()",]
        );
        assert_eq!(Solution::generate_parenthesis(1), ["()",]);
    }
}
