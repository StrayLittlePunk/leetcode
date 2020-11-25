#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn help(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, idx: i32) {
        for i in idx..nums.len() as i32 {
            cur.push(nums[i as usize].clone());
            Self::help(nums, res, cur, i + 1);
            cur.pop();
        }
        res.push(cur.clone());
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::help(&nums, &mut res, &mut Vec::new(), 0);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1],
                vec![2, 3],
                vec![2],
                vec![3],
                vec![]
            ]
        );
    }
}
