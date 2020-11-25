#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(N * M) space O(N + M)
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.len() < 1 || num2.len() < 1 {
            return "".to_string();
        }

        let mut ans = vec![0; num1.len() + num2.len()];
        let (mut i, mut j) = (num1.len(), num2.len());

        for b in num2.chars().rev() {
            for a in num1.chars().rev() {
                let prod = b.to_digit(10).unwrap() * a.to_digit(10).unwrap();
                let sum = prod + ans[i + j - 1];

                ans[i + j - 2] = ans[i + j - 2] + sum / 10;
                ans[i + j - 1] = sum % 10;
                j -= 1;
            }

            j = num2.len();
            i -= 1;
        }

        let ans_str = ans
            .into_iter()
            .map(|d| std::char::from_digit(d, 10).unwrap())
            .skip_while(|c| *c == '0')
            .collect();

        if ans_str == "".to_string() {
            "0".to_string()
        } else {
            ans_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}
