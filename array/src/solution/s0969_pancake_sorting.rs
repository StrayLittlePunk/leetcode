#![allow(unused)]
pub struct Solution {}

// uber interview
impl Solution {
    pub fn pancake_sort(mut a: Vec<i32>) -> Vec<i32> {
      let mut ans = vec![];
        let mut len = a.len();
        while len > 1 {
            let mut max = a[0];
            let mut idx = 0;

            for i in 0..len {
                if a[i] > max {
                    max = a[i];
                    idx = i;
                }
            }
            if idx == len - 1 {
                len -= 1;
                continue;
            }

            ans.push(idx as i32 + 1);
            a[0..=idx].reverse();
            ans.push(len as i32);
            a[0..len].reverse();
            len -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_969() {
    }
}
