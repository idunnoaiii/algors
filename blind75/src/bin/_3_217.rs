use blind75::util::ListNode;

// use the push to array and then init the linkedlist from that array
fn _merge_two_linked_list(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, Some(list2)) => Some(list2.clone()),
        (Some(list1), None) => Some(list1.clone()),
        (Some(list1), Some(list2)) => {
            let mut cur1 = Some(list1);
            let mut cur2 = Some(list2);
            let mut result_vec = vec![];

            while let (Some(cur_node1), Some(cur_node2)) = (cur1.clone(), cur2.clone()) {
                if cur_node1.val <= cur_node2.val {
                    result_vec.push(cur_node1.val);
                    cur1 = cur_node1.next;
                } else {
                    result_vec.push(cur_node2.val);
                    cur2 = cur_node2.next;
                }
            }

            while let Some(cur_node1) = cur1 {
                result_vec.push(cur_node1.val);
                cur1 = cur_node1.next;
            }

            while let Some(cur_node2) = cur2 {
                result_vec.push(cur_node2.val);
                cur2 = cur_node2.next;
            }

            ListNode::from_vec(&result_vec)
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let list1 = ListNode::from_vec(&vec![1, 2, 4]);
        let list2 = ListNode::from_vec(&vec![1, 3, 4]);

        let mut merged = _merge_two_linked_list(list1, list2);

        let check = vec![1, 1, 2, 3, 4, 4];

        let mut i = 0;

        while let Some(entry) = merged {
            assert_eq!(entry.val, check[i]);
            merged = entry.next;
            i += 1;
        }

        assert_eq!(i, check.len())
    }
}
