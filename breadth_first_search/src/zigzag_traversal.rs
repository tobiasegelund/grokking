use crate::Node;
use std::collections::VecDeque;

fn zigzag_traversal(root: Node) -> Vec<VecDeque<i32>> {
    let mut results = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);
    let mut left_to_right = true;

    while queue.len() > 0 {
        let level = queue.len();
        let mut current_level = VecDeque::new();

        for i in 0..level {
            let node = queue.pop_front();
            let unwrap = node.unwrap();

            if !left_to_right {
                current_level.push_front(unwrap.value);
            } else {
                current_level.push_back(unwrap.value);
            }

            if let Some(left) = unwrap.left {
                queue.push_back(*left)
            }
            if let Some(right) = unwrap.right {
                queue.push_back(*right)
            }
        }
        results.push(current_level);
        left_to_right = !left_to_right;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_traversal() {
        let mut root = Node::new(12, None, None);
        root.left = Some(Box::new(Node::new(7, None, None)));
        root.right = Some(Box::new(Node::new(1, None, None)));
        root.left.as_mut().unwrap().left = Some(Box::new(Node::new(9, None, None)));
        root.left.as_mut().unwrap().right = Some(Box::new(Node::new(10, None, None)));
        root.right.as_mut().unwrap().right = Some(Box::new(Node::new(5, None, None)));

        let expected = vec![vec![12], vec![1, 7], vec![9, 10, 5]];

        let got = zigzag_traversal(root);

        assert_eq!(got, expected);
    }
}
