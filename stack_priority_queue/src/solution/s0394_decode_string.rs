#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(N), Space O(N)
    // 对称性考虑用栈 此题用 [ 和] 实现对称性，]是弹栈的时机
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, usize)> = Vec::new();
        let mut cur_num: usize = 0;
        let mut cur_string: String = String::new();

        for l in s.chars() {
            if l == '[' {
                stack.push((cur_string, cur_num));
                cur_string = String::new();
                cur_num = 0;
            } else if l == ']' {
                let (mut string, nums) = stack.pop().unwrap();
                cur_string = cur_string.repeat(nums);
                string.push_str(cur_string.as_str());
                cur_string = string
            } else if l.is_digit(10) {
                cur_num = cur_num * 10 + (l.to_digit(10).unwrap() as usize);
            } else {
                cur_string.push(l)
            }
        }
        cur_string
        // stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_394() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            Solution::decode_string("abc3[cd]xyz".to_string()),
            "abccdcdcdxyz".to_string()
        );
    }
}
