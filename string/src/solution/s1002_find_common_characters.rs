#![allow(unused)]
pub struct Solution {}
// apple interview

impl Solution {
    fn count(s: &str) -> [u8; 26] {
        s.chars()
            .map(|ch| (ch as usize) - 97)
            .fold([0u8; 26], |mut counts, ch| {
                counts[ch] += 1;
                counts
            })
    }
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut answer = Self::count(&a[0]);
        for s in a[1..].iter() {
            let c = Self::count(s);
            for (idx, cnt) in answer.iter_mut().enumerate() {
                *cnt = std::cmp::min(*cnt, c[idx]);
            }
        }
        answer
            .iter()
            .enumerate()
            .flat_map(|(idx, cnt)| std::iter::repeat((idx + 97) as u8 as char).take(*cnt as usize))
            .map(|ch| ch.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1002() {}
}
