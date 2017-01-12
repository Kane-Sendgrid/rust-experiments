pub struct StackQueue<T> {
    stack1: Vec<T>,
    stack2: Vec<T>,
}

impl<T> StackQueue<T> {
    pub fn new() -> Self {
        StackQueue {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    pub fn put(&mut self, item: T) {
        self.stack1.push(item)
    }

    pub fn get(&mut self) -> Option<T> {
        if self.stack2.len() > 0 {
            return self.stack2.pop();
        }
        if self.stack1.len() == 0 {
            return None;
        }
        while !self.stack1.is_empty() {
            self.stack2.push(self.stack1.pop().unwrap());
        }
        self.stack2.pop()
    }
}

#[cfg(test)]
mod test {
    use super::StackQueue;

    #[test]
    fn queue34() {
        println!("queue34");
        let mut q: StackQueue<i32> = StackQueue::new();
        q.put(1);
        q.put(2);
        q.put(3);
        assert_eq!(q.get(), Some(1));
        assert_eq!(q.get(), Some(2));
        assert_eq!(q.get(), Some(3));
        assert_eq!(q.get(), None);
    }
}