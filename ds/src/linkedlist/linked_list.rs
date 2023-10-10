use std::{
    cell::RefCell,
    rc::{self, Rc},
};

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    pub next: SingleLink,
    pub value: String,
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Debug)]
pub struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub len: u16,
}

impl TransactionLog {

    pub fn new_empty() -> Self {
        TransactionLog {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old) => (*old).borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }

        self.len += 1;
        self.tail = Some(new_node);
    }

    // pub fn pop(&mut self) -> Option<String> {
    //     self.head.take().map(|head| {
    //         if let Some(next) = (*head).borrow_mut().next.take() {
    //             self.head = Some(next);
    //         } else {
    //             self.tail.take();
    //         }

    //         self.len -= 1;

    //         Rc::try_unwrap(head)
    //             .ok()
    //             .expect("someting wrong")
    //             .into_inner()
    //             .value

    //         // (*head).borrow_mut().value.clone()
    //     });

    //     None
    // }

    pub fn pop2(&mut self) -> Option<String> {
        self.head.take().map(|cur_head| {

            self.head = match (*cur_head).borrow().next {
                Some(ref next) => Some(next.clone()),
                None => None,
            };

            self.len -= 1;

            // (*cur_head).into_inner().value
            Rc::try_unwrap(cur_head)
                .ok()
                .expect("something wrong")
                .into_inner()
                .value
        });

        None
    }
}

