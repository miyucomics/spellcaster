#![warn(clippy::pedantic)]

pub struct TrieNode {
    pub children: [Option<Box<TrieNode>>; 26],
    pub is_terminal: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_terminal: false,
        }
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for &byte in word.as_bytes() {
            let idx = (byte - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_terminal = true;
    }
}
