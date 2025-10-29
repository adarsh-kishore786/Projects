use std::fmt::Display;

pub struct HuffmanNode<T> {
    pub left: Option<Box<HuffmanNode<T>>>,
    pub right: Option<Box<HuffmanNode<T>>>,
    pub val: T
}

impl<T> HuffmanNode<T> {
    pub fn new(value: T) -> Self {
        HuffmanNode { 
            left: None,
            right: None,
            val: value 
        }
    }
}

impl<T: Display> HuffmanNode<T> {
    fn print_inorder(&self) {
        if let Some(left) = &self.left {
            left.print_inorder();
        }
        println!("{}", &self.val);
        if let Some(right) = &self.right {
            right.print_inorder();
        }
    }
}
