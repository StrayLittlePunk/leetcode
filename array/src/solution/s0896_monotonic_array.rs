#![allow(unused)]
pub struct Solution {}

// facebook interview
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let (mut increasing, mut decreasing) = (true, true);
        for i in 0..a.len() - 1 {
            if a[i] > a[i + 1] {
                increasing = false;
            } else if a[i] < a[i + 1] {
                decreasing = false;
            }
        }

        return increasing || decreasing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_896() {}
}
