#![warn(clippy::pedantic)]

pub struct TrieNode {
    pub children: [Option<usize>; 26],
    pub is_terminal: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: [None; 26],
            is_terminal: false,
        }
    }
}

pub struct Trie {
    pub nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![TrieNode::new()],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut cursor = 0;
        for &byte in word.as_bytes() {
            let character = (byte - b'a') as usize;
            if let Some(next) = self.nodes[cursor].children[character] {
                cursor = next;
            } else {
                let new = self.nodes.len();
                self.nodes.push(TrieNode::new());
                self.nodes[cursor].children[character] = Some(new);
                cursor = new;
            }
        }

        self.nodes[cursor].is_terminal = true;
    }
}
