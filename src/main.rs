use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

impl Node {
    fn new(links: Vec<Link>, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: links,
            offset: offset,
            command: command,
        }))
    }
}

#[derive(Debug, Clone)]
struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl BestTransactionLog {
    pub fn new_empty(max_level: usize) -> BestTransactionLog {
        BestTransactionLog {
            max_level: max_level,
            head: None,
            tails: vec![None; max_level + 1],
            length: 0,
        }
    }

    pub fn append(&mut self, offset: u64, command: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let new = Node::new(vec![None; level], offset, command);
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && self.max_level < n {
            n += 1;
        }
        n
    }
}

fn main() {}
