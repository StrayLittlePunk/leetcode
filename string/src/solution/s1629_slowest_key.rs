#![allow(unused)]
pub struct Solution {}
// amazon interview
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let chs = keys_pressed.chars().collect::<Vec<char>>();
        let mut ch = chs[0];
        let mut ans = release_times[0];
        use std::cmp::max;

        for i in 1..release_times.len() {
            let difference = release_times[i] - release_times[i - 1];
            // release time > prev one , update char
            if difference > ans || difference == ans && chs[i] > ch {
                ch = chs[i];
                ans = difference;
            }
        }

        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1629() {}
}
