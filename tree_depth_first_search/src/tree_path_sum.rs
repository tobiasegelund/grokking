use crate::Node;

fn has_path(root: Option<Box<Node>>, sum: i32) -> bool {
    if root.is_none() {
        return false;
    }

    let node = root.unwrap();

    if node.value == sum && node.left.is_none() && node.right.is_none() {
        return true;
    }

    let new_sum = sum - node.value;

    has_path(node.left, new_sum) || has_path(node.right, new_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_path() {
        let mut root = Node::new(12, None, None);
        root.left = Some(Box::new(Node::new(7, None, None)));
        root.right = Some(Box::new(Node::new(1, None, None)));
        root.left.as_mut().unwrap().left = Some(Box::new(Node::new(9, None, None)));
        root.right.as_mut().unwrap().left = Some(Box::new(Node::new(10, None, None)));
        root.right.as_mut().unwrap().right = Some(Box::new(Node::new(5, None, None)));

        let got = has_path(Some(Box::new(root)), 23);

        let expected = true;

        assert_eq!(got, expected);
    }
}
