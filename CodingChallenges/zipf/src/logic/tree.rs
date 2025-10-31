pub struct HuffmanNode {
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    freq: u32,
    val: Option<char>
}

impl HuffmanNode {
    pub fn new(freq: u32, ch: char) -> Self {
        HuffmanNode { 
            left: None,
            right: None,
            freq: freq,
            val: Some(ch)
        }
    }

    pub fn add_left(mut self, node: HuffmanNode) -> Self {
        self.freq += node.freq;
        self.left = Some(Box::new(node));
        return self;
    }
    
    pub fn add_right(mut self, node: HuffmanNode) -> Self {
        self.freq += node.freq;
        self.right = Some(Box::new(node));
        return self;
    }
}

impl HuffmanNode {
    pub fn print_inorder(&self) {
        if let Some(left) = &self.left {
            left.print_inorder();
        }
        
        print!("{}", self.freq);

        if let Some(val) = self.val {
            println!(": {}", val);
        } else {
            println!("");
        }
        if let Some(right) = &self.right {
            right.print_inorder();
        }
    }
}
