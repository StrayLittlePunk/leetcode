#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut modify = 0;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                modify += 1;
                if i == 1 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
            }
        }

        modify <= 1
    }
}

#[test]
fn testcase() {
    let v = vec![4, 2, 3];
    assert_eq!(true, Solution::check_possibility(v));

    let v = vec![4, 2, 1];
    assert_eq!(false, Solution::check_possibility(v));

    let v = vec![3, 4, 2, 3];
    assert_eq!(false, Solution::check_possibility(v));

    let v = vec![1, 1, 1];
    assert_eq!(true, Solution::check_possibility(v));

    let v = vec![2, 3, 3, 2, 2];
    assert_eq!(false, Solution::check_possibility(v));
}
