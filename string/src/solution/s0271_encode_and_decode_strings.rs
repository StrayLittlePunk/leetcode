#![allow(unused)]

struct Codec {
    delimiter: &'static str,
    delimiter_null: &'static str,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            delimiter: "我",
            delimiter_null: "你",
        }
    }

    fn encode(&self, strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return self.delimiter_null.to_string();
        }

        strs.join(self.delimiter)
    }

    fn decode(&self, s: String) -> Vec<String> {
        if s == self.delimiter_null.to_string() {
            return vec![];
        }

        s.split(self.delimiter)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_271() {
        let strs = vec![
            "leetcode is good".to_string(),
            "codec object will".to_string(),
            "instantiated allowed".to_string(),
        ];

        let obj = Codec::new();
        let s: String = obj.encode(strs);
        println!("{}", s);
        let ans: Vec<String> = obj.decode(s);
        assert_eq!(
            ans,
            vec![
                "leetcode is good".to_string(),
                "codec object will".to_string(),
                "instantiated allowed".to_string()
            ]
        );
    }
}
