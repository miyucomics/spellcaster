use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<u8, TrieNode>,
    pub is_terminal: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
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
            node = node.children.entry(byte).or_insert_with(TrieNode::new);
        }
        node.is_terminal = true;
    }
}
