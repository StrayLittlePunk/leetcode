pub struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result: Vec<String> = vec![s.clone()];

        for (i, ch) in s.chars().enumerate() {
            if ch.is_ascii_alphabetic() {
                if ch.is_ascii_lowercase() {
                    let len = result.len();
                    for j in 0..len {
                        result.push(format!(
                            "{}{}{}",
                            &result[j][0..i],
                            ch.to_ascii_uppercase(),
                            &s[i + 1..]
                        ));
                    }
                } else {
                    let len = result.len();
                    for j in 0..len {
                        result.push(format!(
                            "{}{}{}",
                            &result[j][0..i],
                            ch.to_ascii_lowercase(),
                            &s[i + 1..]
                        ));
                    }
                }
            }
        }

        result
    }
}
