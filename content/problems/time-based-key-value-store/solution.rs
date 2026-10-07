struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap { store: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.store.get(&key) {
            None => String::new(),
            Some(entries) => {
                let i = entries.partition_point(|&(t, _)| t <= timestamp);
                if i == 0 {
                    String::new()
                } else {
                    entries[i - 1].1.clone()
                }
            }
        }
    }
}

/*
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
