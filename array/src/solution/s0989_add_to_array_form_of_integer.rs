
#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn add_to_array_form(mut a: Vec<i32>, k: i32) -> Vec<i32> {
        let n = a.len();
        let mut i = n as i32 - 1;
        let mut ans = vec![];
        let mut cur = k;
        while i >= 0 || cur > 0 {
            if i >= 0 {
                cur += a[i as usize];    
            }
            
            ans.push(cur % 10);
            cur /= 10;
            i -= 1;
        }
        
        ans.reverse();
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_989() {
    }
}
