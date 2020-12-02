#![allow(unused)]
pub struct Solution {}

// google interview
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        emails
            .into_iter()
            .map(Self::clean_email)
            .collect::<HashSet<_>>()
            .len() as i32
    }

    fn clean_email(email: String) -> String {
        let v: Vec<&str> = email.split('@').collect();

        let left = v[0]
            .chars()
            .take_while(|&c| c != '+')
            .filter(|&c| c != '.')
            .collect::<String>();
        [&left, v[1]].join("@")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_929() {}
}
