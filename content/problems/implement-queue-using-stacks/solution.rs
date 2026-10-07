struct MyQueue {
    inbox: Vec<i32>,
    outbox: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue { inbox: Vec::new(), outbox: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.inbox.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.peek();
        self.outbox.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.outbox.is_empty() {
            while let Some(x) = self.inbox.pop() {
                self.outbox.push(x);
            }
        }
        *self.outbox.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}
