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

    pub fn add_left(mut self, node: HuffmanNode<T>) -> Self {
        self.left = Some(Box::new(node));
        return self;
    }
    
    pub fn add_right(mut self, node: HuffmanNode<T>) -> Self {
        self.right = Some(Box::new(node));
        return self;
    }
}

impl<T: Display> HuffmanNode<T> {
    pub fn print_inorder(&self) {
        if let Some(left) = &self.left {
            left.print_inorder();
        }
        println!("{}", &self.val);
        if let Some(right) = &self.right {
            right.print_inorder();
        }
    }
}
