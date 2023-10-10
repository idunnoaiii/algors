use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
pub struct Node {
    pub prev: Link,
    pub next: Link,
    pub value: String,
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct BetterLogTransaction {
    pub head: Link,
    pub tail: Link,
    pub len: u64,
}

impl BetterLogTransaction {
    pub fn new_empty() -> Self {
        BetterLogTransaction {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                (*old).borrow_mut().next = Some(new_node.clone());
            }
            None => self.head = Some(new_node.clone()),
        };

        self.len += 1;
        self.tail = Some(new_node.clone());
    }
}

pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    pub fn new(start_at: Link) -> ListIterator {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current = &self.current;

        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };

        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };

        result
    }
}
