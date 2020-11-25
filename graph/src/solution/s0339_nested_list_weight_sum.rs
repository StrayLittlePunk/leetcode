#![allow(unused)]
pub struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    //   Complexity Analysis
    //
    // The algorithm takes O(N) time, where N is the total number of nested elements
    // in the input list. For example, the list [ [[[[1]]]], 2 ] contains 4 nested lists and 2
    // nested integers (1 and 2), so N=6.
    //
    // In terms of space, at most O(D) recursive calls are placed on the stack, where D
    // is the maximum level of nesting in the input. For example, D=2
    // for the input [[1,1],2,[1,1]], and D=3 for the input [1,[4,[6]]].
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        if nested_list.len() < 1 {
            return 0;
        }

        fn helper(nested_int: NestedInteger, depth: i32) -> i32 {
            match nested_int {
                NestedInteger::Int(c) => depth * c,
                NestedInteger::List(vec) => {
                    let mut sum = 0;
                    for item in vec {
                        sum += helper(item, depth + 1);
                    }

                    sum
                }
            }
        }

        let mut ans = 0;
        for nested_int in nested_list {
            ans += helper(nested_int, 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_339() {
        assert_eq!(
            Solution::depth_sum(vec![
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1),]),
                NestedInteger::Int(2),
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1),])
            ]),
            10
        );

        assert_eq!(
            Solution::depth_sum(vec![
                NestedInteger::Int(1),
                NestedInteger::List(vec![
                    NestedInteger::Int(4),
                    NestedInteger::List(vec![NestedInteger::Int(6)])
                ])
            ]),
            27
        );
    }
}
