#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_self_crossing(x: Vec<i32>) -> bool {
        let n = x.len();
        if n < 4 {
            return false;
        }

        for i in 3..n {
            if x[i] >= x[i - 2] && x[i - 1] <= x[i - 3] {
                return true;
            }
            if i >= 4 {
                if x[i - 1] == x[i - 3] && x[i] + x[i - 4] >= x[i - 2] {
                    return true;
                }
            }
            if i >= 5 {
                if x[i - 2] - x[i - 4] >= 0
                    && x[i] >= x[i - 2] - x[i - 4]
                    && x[i - 1] >= x[i - 3] - x[i - 5]
                    && x[i - 1] <= x[i - 3]
                {
                    return true;
                }
            }
        }
        return false;
    }
}

// Categorize the self-crossing scenarios, there are 3 of them:
// 1. Fourth line crosses first line and works for fifth line crosses second line and so on...
// 2. Fifth line meets first line and works for the lines after
// 3. Sixth line crosses first line and works for the lines after
/*               i-2
    case 1 : i-1┌─┐
                └─┼─>i
                 i-3

                    i-2
    case 2 : i-1 ┌────┐
                 └─══>┘i-3
                 i  i-4      (i overlapped i-4)

    case 3 :    i-4
               ┌──┐
               │i<┼─┐
            i-3│ i-5│i-1
               └────┘
                i-2

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_399() {
        // code here
        assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2,]), true);
        assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 1,]), true);
    }
}
