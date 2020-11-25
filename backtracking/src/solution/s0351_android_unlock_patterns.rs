#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut ret = 0;
        let mut used = vec![false; 9];
        for i in m..=n {
            ret += Self::calc_patterns(-1, i as i32, &mut used);
            for k in 0..9 {
                used[k] = false;
            }
        }

        ret
    }

    fn calc_patterns(last: i32, len: i32, used: &mut Vec<bool>) -> i32 {
        if len == 0 {
            return 1;
        }

        let mut sum = 0;
        for i in 0..9 {
            if Self::is_valid(i, last, used) {
                used[i as usize] = true;
                sum += Self::calc_patterns(i, len - 1, used);
                used[i as usize] = false;
            }
        }

        return sum;
    }

    fn is_valid(index: i32, last: i32, used: &mut Vec<bool>) -> bool {
        if used[index as usize] {
            return false;
        }

        if last == -1 {
            return true;
        }

        if (index + last) % 2 == 1 {
            return true;
        }

        let mid = (index + last) / 2;
        if mid == 4 {
            return used[mid as usize];
        }

        if index % 3 != last % 3 && index / 3 != last / 3 {
            return true;
        }

        return used[mid as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_351() {}
}
