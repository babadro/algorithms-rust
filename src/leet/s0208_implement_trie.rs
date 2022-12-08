struct Trie {
    root: Option<Box<Node>>,
}

#[derive(Clone)]
struct Node {
    children: Vec<Option<Box<Node>>>,
    end_word: bool,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            root: Some(Box::new(Node {
                children: vec![None; 26],
                end_word: true,
            })),
        }
    }

    fn insert(&self, word: String) {
        let mut cur = &self.root;
        for i in 0..word.len() {
            let ch = (word.as_bytes()[i] - b'a') as usize;
        }
    }

    fn search(&self, word: String) -> bool {
        false //todo
    }

    fn starts_with(&self, prefix: String) -> bool {
        false
    }
}
