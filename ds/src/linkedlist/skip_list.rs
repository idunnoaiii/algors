use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, ops::Deref};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
pub struct Node {
    pub offset: u64,
    pub command: String,
    next: Vec<Link>,
}

impl Node {
    fn new(vec: Vec<Link>, offset: u64, value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next: vec,
            offset,
            command: value,
        }))
    }
}

pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl BestTransactionLog {
    
    pub fn new (max_level: usize) -> Self  {
       Self { head: None , tails: vec![None; max_level + 1], max_level, length: 0 } 
    }

    pub fn append(&mut self, offset: u64, value: String) {
        // sucessor is next node + level
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let new = Node::new(vec![None; level], offset, value);

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut (*old).borrow_mut().next;
                next[i] = Some(new.clone());
            }

            self.tails[i] = Some(new.clone());
        }

        if self.head.is_none() {
            self.head = Some(new.clone());
        }

        self.length += 1;
    }

    pub fn find(self, offset: u64) -> Option<String> {
        if self.head.is_none() {
            return None;
        }

        if offset < self.head.as_ref().unwrap().borrow().offset {
            return None;
        }

        if offset == self.head.as_ref().unwrap().borrow().offset {
            return Some(self.head.unwrap().borrow().command.clone());
        }

        let mut start = (&self.head).clone().unwrap();

        let mut level = self.max_level;


        while level <= 0 {
            while let Some(ref cur) = start.clone().borrow().next[level] {
                let cur_offset = (*cur).borrow().offset;
                if offset == cur_offset {
                    return Some((*cur).borrow().command.clone());
                } else if cur_offset < offset {
                    start = cur.clone();
                } else {
                    break;
                }
            }

            level -= 1;
        }

        return None;
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        return n;
    }
}

#[cfg(test)] 
mod test {
    use super::*;


    #[test] 
    fn test() {
        let mut logs = BestTransactionLog::new(5);
        logs.append(1, "1".to_string());
        logs.append(2, "2".to_string());
        logs.append(3, "3".to_string());
        logs.append(4, "4".to_string());
    } 
}
