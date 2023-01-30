mod tree_path_sum;

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node { value, left, right }
    }
}
