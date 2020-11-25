#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(1) Space O(1)
    // 对所有数字执行按位与运算的结果是所有对应二进制字符串的公共前缀再用零补上后面的剩余位。
    // 计算两个二进制字符串的公共前缀。
    pub fn range_bitwise_and_shift(mut m: i32, mut n: i32) -> i32 {
        let mut shift = 0;
        while m < n {
            m >>= 1;
            n >>= 1;
            shift += 1;
        }

        m << shift
    }
    // 一个位移相关的算法叫做「Brian Kernighan 算法」，它用于清除二进制串中最右边的 1。
    // Brian Kernighan 算法的关键在于我们每次对 number 和 number−1 之间进行按位与运算后，
    // number 中最右边的 1 会被抹去变成 0。
    // 计算两个二进制字符串的公共前缀。
    // 其思想是，对于给定的范围 [m,n]（m<n），我们可以对数字 n 迭代地应用上述技巧
    // 清除最右边的 1，直到它小于或等于 m，此时非公共前缀部分的 1 均被消去。因此最后我们返回 n 即可

    // Time O(1) Space O(1)
    pub fn range_bitwise_and(m: i32, mut n: i32) -> i32 {
        while m < n {
            n &= n - 1;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_201() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(1, 3), 0);
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
}
