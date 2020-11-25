#![allow(unused)]
pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    // Time O(N), Space O(N) Iterative Solution
    pub fn deserialize(s: String) -> NestedInteger {
        if !&s.starts_with("[") {
            return NestedInteger::Int(s.parse::<i32>().unwrap());
        }

        let mut stack: Vec<NestedInteger> = vec![];
        let mut digit_str: String = String::new();
        for c in s.chars() {
            if c == '[' {
                stack.push(NestedInteger::List(vec![]));
            } else if c == '-' || c.is_digit(10) {
                digit_str.push(c);
            } else if c == ',' {
                if !digit_str.is_empty() {
                    if let Some(v) = stack.last_mut() {
                        if let NestedInteger::List(n) = v {
                            n.push(NestedInteger::Int(digit_str.parse::<i32>().unwrap()));
                        }
                    }
                    digit_str.truncate(0);
                }
            } else {
                if !digit_str.is_empty() {
                    if let Some(v) = stack.last_mut() {
                        if let NestedInteger::List(n) = v {
                            n.push(NestedInteger::Int(digit_str.parse::<i32>().unwrap()));
                        }
                    }
                    digit_str.truncate(0);
                }
                let n = stack.pop().unwrap();
                if stack.is_empty() {
                    return n;
                } else if let Some(v) = stack.last_mut() {
                    if let NestedInteger::List(nst) = v {
                        nst.push(n);
                    }
                }
            }
        }
        NestedInteger::Int(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_385() {
        assert_eq!(
            Solution::deserialize("324".to_string()),
            NestedInteger::Int(324)
        );
        assert_eq!(
            Solution::deserialize("[123,[456,[789]]]".to_string()),
            NestedInteger::List(vec![
                NestedInteger::Int(123),
                NestedInteger::List(vec![
                    NestedInteger::Int(456),
                    NestedInteger::List(vec![NestedInteger::Int(789)])
                ])
            ])
        );
    }
}
