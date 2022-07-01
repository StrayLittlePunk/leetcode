#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn reverse_words(sen: String) -> String {
        let mut ans = String::default();
        for s in sen.split_whitespace() {
            for c in s.chars().rev() {
                ans.push(c);
            }
            ans.push(' ');
        }

        ans.pop();
        ans
    }
}

#[test]
fn testcase() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
    assert_eq!(
        Solution::reverse_words("God Ding".to_string()),
        "doG gniD".to_string()
    );
}
