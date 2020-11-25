#![allow(unused)]
pub struct Solution {}
// microsoft interview
impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let (cha, chb) = (
            a.chars().collect::<Vec<char>>(),
            b.chars().collect::<Vec<char>>(),
        );
        // a len not equal to b , can't shift
        if cha.len() != chb.len() {
            return false;
        }

        if cha.len() == 0 {
            return true;
        }

        for i in 0..chb.len() {
            // i represent partition index
            if chb[i] != cha[0] {
                continue;
            }

            let mut j = i + 1;
            while j % chb.len() != i {
                // b string compare failed a string, jump out loop
                if chb[j % chb.len()] != cha[j - i] {
                    break;
                }
                j += 1;
            }
            // if b string compare success a string, run all the loop ,and j % chb.len() == i
            if j % chb.len() == i {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_796() {}
}
