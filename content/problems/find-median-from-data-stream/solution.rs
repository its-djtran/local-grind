struct MedianFinder {
    lo: BinaryHeap<i32>,          // max-heap of the smaller half
    hi: BinaryHeap<Reverse<i32>>, // min-heap of the larger half
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder { lo: BinaryHeap::new(), hi: BinaryHeap::new() }
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        let top = self.lo.pop().unwrap();
        self.hi.push(Reverse(top));
        if self.hi.len() > self.lo.len() {
            let Reverse(x) = self.hi.pop().unwrap();
            self.lo.push(x);
        }
    }

    fn find_median(&self) -> f64 {
        let a = *self.lo.peek().unwrap() as f64;
        if self.lo.len() > self.hi.len() {
            a
        } else {
            (a + self.hi.peek().unwrap().0 as f64) / 2.0
        }
    }
}
