#![allow(unused)]
pub struct Solution {}

// microsoft interview

impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        use std::cmp::max;
        let (mut right, mut ret, n) = (0, 0, light.len());
        for i in 0..n {
            // 遍历数组，记录当前最大亮起来的灯，如果最大亮起来的灯等于遍历过的灯的数量，
            // 那么说明前面灯都亮
            right = max(right, light[i]);
            if right as usize == i + 1 {
                ret += 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1375() {}
}
