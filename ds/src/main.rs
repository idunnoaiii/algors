use double_linked_list::{Node, BetterLogTransaction, ListIterator};
use std::{cell::RefCell, rc::Rc};

mod double_linked_list;

fn main() {
    let mut log = BetterLogTransaction::new_empty();
    log.append(String::from("1"));
    log.append(String::from("2"));
    log.append(String::from("3"));
    log.append(String::from("4"));

    let mut log_iter = ListIterator::new(log.head);

    for item in log_iter {
        println!("{:?}", item);
    }

}
