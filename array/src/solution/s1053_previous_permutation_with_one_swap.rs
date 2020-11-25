#![allow(unused)]
pub struct Solution {}

// microsoft interview

use std::usize::MAX;
impl Solution {
    pub fn prev_perm_opt1(mut a: Vec<i32>) -> Vec<i32> {
        if a.len() <= 1 {
            return a;
        }
        let mut idx = MAX;
        // find the largest i such that a[i] > a[i + 1]
        for i in (1..a.len()).rev() {
            if a[i] < a[i - 1] {
                idx = i - 1;
                break;
            }
        }
        // the array already sorted, not smaller permutation
        if idx == MAX {
            return a;
        }
        // find the largest j such that a[j] > a[i], then swap them
        for i in (idx as usize..a.len()).rev() {
            // the second check to skip duplicate numbers
            if a[i] < a[idx] && a[i] != a[i -1] {
                a.swap(i, idx);
                break;
            }
        }
        a
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1053() {
    }
}
