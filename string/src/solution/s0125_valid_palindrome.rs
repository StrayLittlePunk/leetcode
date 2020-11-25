#![allow(unused)]
pub struct Solution {}
// facebook interview

impl Solution {
  // O(N) O(1)
    pub fn is_palindrome(s: String) -> bool {
        let chs = s.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, chs.len() as i32 - 1);

        while left < right {
            while left < right && !chs[left as usize].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !chs[right as usize].is_ascii_alphanumeric() {
                right -= 1;
            }

            if left < right
                && chs[left as usize].to_ascii_lowercase()
                    != chs[right as usize].to_ascii_lowercase()
            {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
     fn is_palindrome_func(s: String) -> bool {
        let s: String = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
        s.chars().zip(s.chars().rev()).fold(true, |acc, (a, b)| acc & (a == b))
    }
     fn is_palindrome_f(mut s: String) -> bool {
        s.make_ascii_lowercase();
        let mut chars = s.chars().filter(char::is_ascii_alphanumeric);
        while let (Some(next), Some(next_back)) = (chars.next(), chars.next_back()) {
            if next != next_back {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
    }
}
