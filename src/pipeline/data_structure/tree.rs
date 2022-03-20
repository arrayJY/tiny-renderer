#[derive(Debug, Default, Clone)]
pub struct Node<T: Default> {
    pub val: T,
    pub l: Option<Box<Node<T>>>,
    pub r: Option<Box<Node<T>>>,
}

pub struct Tree<T: Default> {
    pub root: Node<T>
}
