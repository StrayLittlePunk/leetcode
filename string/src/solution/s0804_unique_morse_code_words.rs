#![allow(unused)]
pub struct Solution {}
// apple interview
use std::collections::HashSet;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut set = HashSet::new();
        for word in words.iter() {
            let mut code = String::from("");
            for byte in word.as_bytes() {
                code.push_str(morse[(byte - 97) as usize]);
            }
            set.insert(code);
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_804() {}
}
