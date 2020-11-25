#![allow(unused)]
// google interview
pub struct Solution {}
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut ret = 0;

        for up in 0..m {
            let mut high = vec![1;n];
            for down in up..m {
                for k in 0..n {
                    high[k] &= mat[down][k];

                }
                ret += Self::count_one_row(&high);
            }
        }

        ret
    }

    fn count_one_row(arr: &Vec<i32>) -> i32 {
        let (mut ret, mut len) = (0, 0);
        for i in 0..arr.len() {
            len = if arr[i] == 0 {0} else {len +1};
            ret += len;
        }

        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1504() {

    }
}
