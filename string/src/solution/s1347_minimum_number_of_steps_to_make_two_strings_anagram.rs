#![allow(unused)]
pub struct Solution {}
// microsoft interview

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let (mut chs, mut cht, mut ans) = ([0; 26], [0; 26], 0);
        for (a, b) in s.chars().zip(t.chars()) {
            chs[a as u8 as usize - 97] += 1;
            cht[b as u8 as usize - 97] += 1;
        }

        for i in 0..chs.len() {
            if chs[i] == 0 {
                continue;
            }
            //remain ans need to replace
            if chs[i] - cht[i] > 0 {
                ans += chs[i] - cht[i];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1347() {}
}
