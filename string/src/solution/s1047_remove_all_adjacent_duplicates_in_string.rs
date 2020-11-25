#![allow(unused)]
pub struct Solution {}
// google interview
//
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut chs = s.chars().collect::<Vec<char>>();
        let mut ans = vec![];

        for i in 0..chs.len() {
            if let Some(&ch) = ans.last() {
                if ch == chs[i] {
                    ans.pop();
                } else {
                    ans.push(chs[i]);
                }
            } else {
                ans.push(chs[i]);
            }
        }

        ans.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1047() {}
}
