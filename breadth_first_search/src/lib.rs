mod binary_tree_order_traversal;
mod level_averages_in_binary_tree;
mod level_order_successor;
mod minimum_depth_in_binary_tree;
mod reverse_level_order_traversal;
mod zigzag_traversal;

#[derive(Clone)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // TODO: Find out how to use None as default values
    fn new(value: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        return Self { value, left, right };
    }
}
