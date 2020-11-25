#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n^4) O(n^3)
    pub fn is_scramble(s1: String, s2: String) -> bool {
        if s1.len() == 0 {
            return true;
        }
        if s1.len() == 1 {
            return s1 == s2;
        }

        let mut is_scram = vec![vec![vec![false; s1.len()]; s1.len()]; s1.len() + 1];
        let s1_chs = s1.chars().collect::<Vec<char>>();
        let s2_chs = s2.chars().collect::<Vec<char>>();

        for i in 0..s1.len() {
            for j in 0..s1.len() {
                is_scram[1][i][j] = s1_chs[i] == s2_chs[j];
            }
        }

        for len in 2..=s1.len() {
            for i in 0..=s1.len() - len {
                for j in 0..=s1.len() - len {
                    is_scram[len][i][j] = false;
                    for k in 1..len {
                        if is_scram[len][i][j] {
                            break;
                        }

                        is_scram[len][i][j] = is_scram[len][i][j]
                            || (is_scram[k][i][j] && is_scram[len - k][i + k][j + k]);
                        is_scram[len][i][j] = is_scram[len][i][j]
                            || (is_scram[k][i + len - k][j] && is_scram[len - k][i][j + k]);
                    }
                }
            }
        }

        is_scram[s1.len()][0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_87() {
        assert_eq!(
            Solution::is_scramble("great".to_string(), "rgeat".to_string()),
            true
        );
    }
}
