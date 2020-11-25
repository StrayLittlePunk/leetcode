#![allow(unused)]
pub struct Solution {}
// amazon interview
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        Self::gcd_string(&str1, &str2).to_string()
    }
    fn gcd_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1 == str2 {
            return str1;
        } else {
            let (mut strs, mut strl) = (str2, str1);
            if str1.len() < str2.len() {
                strs = str1;
                strl = str2;
            }

            if strs == &strl[0..strs.len()] {
                Self::gcd_string(&strl[strs.len()..], strs)
            } else {
                ""
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1071() {}
}
