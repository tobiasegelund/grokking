use crate::Node;
use std::collections::VecDeque;

fn find_minimum_depth(root: Node) -> usize {
    let mut result = 1;
    let mut queue = VecDeque::new();

    queue.push_back(root);

    while queue.len() > 0 {
        let level = queue.len();

        result += 1;
        for _ in 0..level {
            let node = queue.pop_front().unwrap();

            match node.left {
                Some(left) => queue.push_back(*left),
                None => return result,
            }

            match node.right {
                Some(right) => queue.push_back(*right),
                None => return result,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_minimum_depth() {
        let mut root = Node::new(12, None, None);
        root.left = Some(Box::new(Node::new(7, None, None)));
        root.right = Some(Box::new(Node::new(1, None, None)));
        root.left.as_mut().unwrap().left = Some(Box::new(Node::new(9, None, None)));
        root.right.as_mut().unwrap().left = Some(Box::new(Node::new(10, None, None)));
        root.right.as_mut().unwrap().right = Some(Box::new(Node::new(5, None, None)));

        let expected = 3;

        let got = find_minimum_depth(root);

        assert_eq!(expected, got);
    }
}
