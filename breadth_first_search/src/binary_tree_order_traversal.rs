use std::collections::VecDeque;

use crate::Node;

fn traversal(root: Node) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut queue: VecDeque<Node> = std::collections::VecDeque::new();

    queue.push_back(root);

    while queue.len() > 0 {
        let level = queue.len();
        let mut current: Vec<i32> = Vec::new();

        for _ in 0..level {
            let current_node = queue.pop_front();
            let unwrap = current_node.unwrap();
            current.push(unwrap.value);

            if let Some(left) = unwrap.left {
                queue.push_back(*left.to_owned())
            }

            if let Some(right) = unwrap.right {
                queue.push_back(*right.clone());
            }
        }

        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal() {
        let mut root = Node::new(12, None, None);
        root.left = Some(Box::new(Node::new(7, None, None)));
        root.right = Some(Box::new(Node::new(1, None, None)));
        root.left.as_mut().unwrap().left = Some(Box::new(Node::new(9, None, None)));
        root.left.as_mut().unwrap().right = Some(Box::new(Node::new(10, None, None)));
        root.right.as_mut().unwrap().right = Some(Box::new(Node::new(5, None, None)));

        let expected = vec![vec![12], vec![7, 1], vec![9, 10, 5]];

        let got = traversal(root);

        assert_eq!(expected, got);
    }
}
