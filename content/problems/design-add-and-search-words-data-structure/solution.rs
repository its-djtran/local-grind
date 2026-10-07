#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    end: bool,
}

struct WordDictionary {
    root: Node,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        WordDictionary { root: Node::default() }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
        }
        node.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut level = vec![&self.root];
        for b in word.bytes() {
            level = if b == b'.' {
                level.iter().flat_map(|node| node.children.iter().flatten().map(|c| &**c)).collect()
            } else {
                let i = (b - b'a') as usize;
                level.iter().filter_map(|node| node.children[i].as_deref()).collect()
            };
            if level.is_empty() {
                return false;
            }
        }
        level.iter().any(|node| node.end)
    }
}
