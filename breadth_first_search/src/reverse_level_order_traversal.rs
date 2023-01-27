use crate::Node;
use std::collections::VecDeque;

fn reverse_traversal(root: Node) -> VecDeque<Vec<i32>> {
    let mut results = VecDeque::new();
    let mut queue: VecDeque<Node> = VecDeque::new();

    queue.push_back(root);

    while queue.len() > 0 {
        let level = queue.len();
        let mut current_list = Vec::new();

        for _ in 0..level {
            let mut current_node = queue.pop_front();
            let unwrap = current_node.unwrap();

            current_list.push(unwrap.value);

            if let Some(left) = unwrap.left {
                queue.push_back(*left);
            }

            if let Some(right) = unwrap.right {
                queue.push_back(*right);
            }
        }

        results.push_front(current_list);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_traversal() {
        let mut root = Node::new(12, None, None);
        root.left = Some(Box::new(Node::new(7, None, None)));
        root.right = Some(Box::new(Node::new(1, None, None)));
        root.left.as_mut().unwrap().left = Some(Box::new(Node::new(9, None, None)));
        root.left.as_mut().unwrap().right = Some(Box::new(Node::new(10, None, None)));
        root.right.as_mut().unwrap().right = Some(Box::new(Node::new(5, None, None)));

        let expected = vec![vec![9, 10, 5], vec![7, 1], vec![12]];

        let got = reverse_traversal(root);

        assert_eq!(got, expected);
    }
}
