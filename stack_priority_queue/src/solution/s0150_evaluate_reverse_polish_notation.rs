#![allow(unused)]
pub struct Solution {}
impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        // 对于 vector 不要用remove 因为很慢，删除index所在节点后，
        // 要index右边的节点左移，所以时间复杂度为O(N)，for in 迭代使用
        // 迭代器，时间复杂度O(1)
        let mut stack: Vec<i32> = vec![];
        for token in tokens {
            if !"+-*/".to_string().contains(&token) {
                // string parse to i32 , if not fit data type will cause panic..
                stack.push(token.parse::<i32>().unwrap());
                continue;
            }
            let rhs = stack.pop().unwrap();
            let lhs = stack.pop().unwrap();
            if token == "*" {
                stack.push(lhs * rhs);
            } else if token == "+" {
                stack.push(lhs + rhs);
            } else if token == "-" {
                stack.push(lhs - rhs);
            } else if token == "/" {
                stack.push(lhs / rhs);
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
