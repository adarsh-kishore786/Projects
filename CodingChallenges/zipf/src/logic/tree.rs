pub struct HuffmanNode {
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    pub freq: u32,
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

    pub fn combine(self, other: Self) -> Self {
        return HuffmanNode {
            freq: self.freq + other.freq,
            left: Some(Box::new(self)),
            right: Some(Box::new(other)),
            val: None
        };
    }
}

impl HuffmanNode {
    pub fn print_preorder(&self) {
        print!("{}", self.freq);

        if let Some(val) = self.val {
            println!(": {}", val);
        } else {
            println!("");
        }

        if let Some(left) = &self.left {
            left.print_preorder();
        }
        
        if let Some(right) = &self.right {
            right.print_preorder();
        }
    }
}
