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
     pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        Solution::helper(&input)
    }

    fn helper(input: &str) -> Vec<i32> {
        if input.is_empty() { return vec![] }
        if let Ok(digit) = input.parse::<i32>() {
            return vec![digit]
        }
        let mut res: Vec<i32> = Vec::new();
        for (i, ch) in input.chars().enumerate() {
            if ch == '+' || ch == '-' || ch == '*' {
                let left = Solution::helper(&input[..i]);
                let right = Solution::helper(&input[i+1..]);
                for &a in left.iter() {
                    for &b in right.iter() {
                        res.push(match ch {
                            '+' => a + b,
                            '-' => a - b,
                            '*' => a * b,
                            _ => unreachable!(),
                        })
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_241() {
        assert_eq!(Solution::diff_ways_to_compute("2-1-1".to_owned()), vec![2, 0]);
        assert_eq!(Solution::diff_ways_to_compute("2*3-4*5".to_owned()), vec![-34, -10, -14, -10, 10]);
    }
}
