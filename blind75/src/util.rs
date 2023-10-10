#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None
        }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<Self>> {

        if vec.len() == 0 {
            return None;
        }

        let mut prev_node = None;

        for item in vec.iter().rev() {
            let mut new_node = Self::new(*item);
            new_node.next = prev_node;
            prev_node = Some(Box::new(new_node));
        }

        return Some(Box::new(*prev_node.unwrap()));
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn ex() {
        let mut head = ListNode::from_vec(&vec![1,2,3,4,5]);

        let mut n = 1;

        while let Some(entry) = head {
            assert_eq!(entry.val, n);
            n += 1;
            head = match entry.next  {
                Some(next) => Some(next),
                None => None,
            };
        }

    }
}
