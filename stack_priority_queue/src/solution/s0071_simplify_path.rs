#![allow(unused)]
pub struct Solution {}
impl Solution {
    pub fn simplify_path(mut path: String) -> String {
        
        format!(
            "/{}",
            path.split("/")
                .filter(|x| *x != "" && *x != ".")
                .fold(vec![], |mut s, section| {
                    match section {
                        ".." => match s.last() {
                            Some(&"..") | None => { s.push("..") }
                            _ => { s.pop(); }
                        }
                        x => match s.last() {
                            Some(&"..") => { s.pop(); s.push(x); }
                            _ => { s.push(x) }
                        }
                    };
                    s
                })
                .into_iter()
                .skip_while(|x| *x == "..")
                .collect::<Vec<&str>>()
                .join("/")
        )
    }
    pub fn simplify_path_other(path: String) -> String {
        let mut stack = vec![];
        for part in path.split("/") {
            if part == ".." {
                stack.pop();
            } else if part == "." {
                continue;
            } else {
                if !part.is_empty() {
                    stack.push(part);
                }
            }
        }

        format!("/{}", stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_71() {
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string()
        );
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_string()),
            "/c".to_string()
        );
        assert_eq!(
            Solution::simplify_path("/a/../../b/../c//.//".to_string()),
            "/c".to_string()
        );
        assert_eq!(
            Solution::simplify_path("/a//b////c/d//././/..".to_string()),
            "/a/b/c".to_string()
        );
    }
}
