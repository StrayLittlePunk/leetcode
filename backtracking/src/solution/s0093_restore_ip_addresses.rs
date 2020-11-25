#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(chs: &String, ans: &mut Vec<String>, index: usize, ip: String, count: i32) {
        if count == 4 && index == chs.len() {
            ans.push(ip);
            return;
        }

        for i in 1..=3 {
            if index + i > chs.len() {
                break;
            }

            let sector: String = if let Some(substr) = chs.get(index..index + i) {
                substr.to_string()
            } else {
                "".to_string()
            };

            if (sector.len() > 1 && sector.starts_with("0"))
                || match sector.parse::<i32>() {
                    Ok(i) => i > 255,
                    Err(_) => true,
                }
            {
                break;
            }

            Solution::backtrack(
                chs,
                ans,
                index + i,
                match count {
                    0 => ip.clone() + sector.as_str(),
                    _ => ip.clone() + "." + sector.as_str(),
                },
                count + 1,
            );
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = vec![];
        if s.len() < 4 || s.len() > 12 {
            return ans;
        } else if s.len() == 4 {
            ans.push(
                s.split("")
                    .collect::<Vec<&str>>()
                    .iter()
                    .filter(|s| **s != "")
                    .map(|s| *s)
                    .collect::<Vec<&str>>()
                    .join("."),
            );
            return ans;
        }

        Self::backtrack(&s, &mut ans, 0, "".to_owned(), 0);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_93() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_owned()),
            vec!["255.255.11.135".to_owned(), "255.255.111.35".to_owned(),]
        );
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_owned()),
            vec!["0.0.0.0".to_owned(),]
        );
        assert_eq!(
            Solution::restore_ip_addresses("1111".to_owned()),
            vec!["1.1.1.1".to_owned(),]
        );
        assert_eq!(
            Solution::restore_ip_addresses("010010".to_owned()),
            vec!["0.10.0.10".to_owned(), "0.100.1.0".to_owned(),]
        );
        assert_eq!(
            Solution::restore_ip_addresses("101023".to_owned()),
            vec![
                "1.0.10.23".to_owned(),
                "1.0.102.3".to_owned(),
                "10.1.0.23".to_owned(),
                "10.10.2.3".to_owned(),
                "101.0.2.3".to_owned(),
            ]
        );
    }
}
