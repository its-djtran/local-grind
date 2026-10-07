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

    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
        }
        node.end = true;
    }

    fn walk(&self, s: &str) -> Option<&Trie> {
        let mut node = self;
        for b in s.bytes() {
            node = node.children[(b - b'a') as usize].as_deref()?;
        }
        Some(node)
    }

    fn search(&self, word: String) -> bool {
        self.walk(&word).map_or(false, |node| node.end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.walk(&prefix).is_some()
    }
}
