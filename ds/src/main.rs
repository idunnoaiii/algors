use double_linked_list::{Node, BetterLogTransaction};
use std::{cell::RefCell, rc::Rc};

mod linked_list;
mod double_linked_list;

fn main() {
    let mut log = BetterLogTransaction::new_empty();
    log.append(String::from("1"));
    log.append(String::from("2"));
    log.append(String::from("3"));
    log.append(String::from("4"));
    println!("{:?}", log);
}
