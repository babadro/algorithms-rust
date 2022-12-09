struct Trie {
    root: Node,
}

struct Node {
    children: [Option<Box<Node>>; 26],
    end_word: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: [(); 26].map(|_| Option::<Box<Node>>::default()),
            end_word: false,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for b in word.bytes() {
            let ch = (b - b'a') as usize;

            cur = cur.children[ch].get_or_insert(Box::new(Node::new()))
        }

        cur.end_word = true
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;

        for b in word.bytes() {
            let ch = (b - b'a') as usize;

            if cur.children[ch].is_none() {
                return false;
            }

            cur = cur.children[ch].as_ref().unwrap()
        }

        return cur.end_word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;

        for b in prefix.bytes() {
            let ch = (b - b'a') as usize;

            if cur.children[ch].is_none() {
                return false;
            }

            cur = cur.children[ch].as_ref().unwrap();
        }

        true
    }
}
