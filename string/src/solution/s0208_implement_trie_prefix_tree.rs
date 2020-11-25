#![allow(unused)]

// amazon interview
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut node = self;
        for &c in word.as_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
        }
        node.end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for i in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match cur.children[i].as_ref() {
                Some(node) => {
                    cur = node;
                }
                None => {
                    return false;
                }
            }
        }

        cur.end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for i in prefix.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match cur.children[i].as_ref() {
                Some(node) => {
                    cur = node;
                }
                None => {
                    return false;
                }
            }
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_208() {}
}
