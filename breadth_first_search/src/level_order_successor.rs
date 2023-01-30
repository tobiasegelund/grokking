use crate::Node;
use std::collections::VecDeque;

fn find_level_order_successor(root: Node, val: i32) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut flag = false;

    queue.push_back(root);

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();

        if flag {
            return Some(node.value);
        }

        if node.value == val {
            flag = true;
        }

        if let Some(left) = node.left {
            queue.push_back(*left);
        }

        if let Some(right) = node.right {
            queue.push_back(*right);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_level_order_successor() {
        let mut root = Node::new(12, None, None);
        root.left = Some(Box::new(Node::new(7, None, None)));
        root.right = Some(Box::new(Node::new(1, None, None)));
        root.left.as_mut().unwrap().left = Some(Box::new(Node::new(9, None, None)));
        root.right.as_mut().unwrap().left = Some(Box::new(Node::new(10, None, None)));
        root.right.as_mut().unwrap().right = Some(Box::new(Node::new(5, None, None)));

        let expected = 9;

        let got = find_level_order_successor(root, 1);

        assert_eq!(expected, got.unwrap());
    }
}
