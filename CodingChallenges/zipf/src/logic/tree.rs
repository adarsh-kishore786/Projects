use std::collections::HashMap;

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

#[allow(dead_code)]
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

impl HuffmanNode {
    pub fn get_huffman_codes(&self) -> HashMap<char, String> {
        let mut codes = HashMap::new();
        self.get_huffman_codes_helper(&mut codes, String::new());
        return codes;
    }

    fn get_huffman_codes_helper(&self, codes: &mut HashMap<char, String>, current_code: String) {
        if let Some(val) = self.val {
            codes.insert(val, current_code);
        } else {
            if let Some(left) = &self.left {
                left.get_huffman_codes_helper(codes, format!("{}0", current_code));
            }
            if let Some(right) = &self.right {
                right.get_huffman_codes_helper(codes, format!("{}1", current_code));
            }
        }
    }
}
