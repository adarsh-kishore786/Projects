use std::collections::BTreeMap;

pub struct HuffmanNode {
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    pub freq: u32,
    val: Option<char>
}

// Create new HuffmanNode and combine two nodes
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

// Print the tree in preorder traversal (for debugging)
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

// Get Huffman codes from the tree
impl HuffmanNode {
    pub fn get_huffman_codes(&self) -> BTreeMap<char, String> {
        let mut codes = BTreeMap::new();
        self.get_huffman_codes_helper(&mut codes, String::new());
        return codes;
    }

    fn get_huffman_codes_helper(&self, codes: &mut BTreeMap<char, String>, current_code: String) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_declaration() {
        let node = HuffmanNode::new(5, 'a');
        assert_eq!(node.freq, 5);
        assert_eq!(node.val.unwrap(), 'a');
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_node_combination() {
        let left_child = HuffmanNode::new(5, 'a');
        let right_child = HuffmanNode::new(3, 'b');
        let parent = left_child.combine(right_child);
        assert_eq!(parent.freq, 8);
        assert!(parent.val.is_none());
        assert!(parent.left.is_some());
        assert!(parent.right.is_some());
    }

    #[test]
    fn test_huffman_codes() {
        let left_child = HuffmanNode::new(5, 'a');
        let right_child = HuffmanNode::new(3, 'b');
        let root = left_child.combine(right_child);
        let codes = root.get_huffman_codes();
        assert_eq!(codes.get(&'a').unwrap(), "0");
        assert_eq!(codes.get(&'b').unwrap(), "1");
    }

    #[test]
    fn test_huffman_codes_complex() {
        let node_a = HuffmanNode::new(2, 'a');
        let node_b = HuffmanNode::new(3, 'b');
        let node_c = HuffmanNode::new(5, 'c');
        let combined_ab = node_a.combine(node_b);
        let root = combined_ab.combine(node_c);
        let codes = root.get_huffman_codes();
        assert_eq!(codes.get(&'c').unwrap(), "1");
        assert_eq!(codes.get(&'a').unwrap(), "00");
        assert_eq!(codes.get(&'b').unwrap(), "01");
    }
}
