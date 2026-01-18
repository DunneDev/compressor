use crate::huffman::BYTE_ALPHABET_SIZE;
use crate::huffman::byte_map::{ByteMap, CodeEntry, CodeLength};
use crate::huffman::frequency::Frequencies;
use std::cmp;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub enum HuffmanNode {
    Leaf {
        byte: u8,
        fequency: u64,
    },
    Internal {
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
        frequency: u64,
    },
}

impl HuffmanNode {
    pub fn from_frequencies(frequencies: &Frequencies) -> Self {
        let mut nodes = BinaryHeap::new();

        for (byte, &count) in frequencies.iter().enumerate().take(BYTE_ALPHABET_SIZE) {
            if count > 0 {
                nodes.push(HuffmanNode::Leaf {
                    byte: byte as u8,
                    fequency: count,
                })
            }
        }

        while nodes.len() > 1 {
            let left = Box::new(nodes.pop().unwrap());
            let right = Box::new(nodes.pop().unwrap());
            let frequency = left.frequency() + right.frequency();

            nodes.push(HuffmanNode::Internal {
                left,
                right,
                frequency,
            })
        }

        nodes
            .pop()
            .expect("There should always be exactly one node left after building Huffman tree")
    }

    fn frequency(&self) -> u64 {
        match self {
            HuffmanNode::Leaf { fequency, .. } => *fequency,
            HuffmanNode::Internal { frequency, .. } => *frequency,
        }
    }

    pub fn to_byte_map(&self) -> ByteMap {
        let mut code_lengths = vec![];
        self.traverse(0, &mut code_lengths);

        ByteMap::new(&mut code_lengths)
    }

    fn traverse(&self, mut len: u8, codes: &mut Vec<CodeLength>) {
        match self {
            HuffmanNode::Internal {
                left,
                right,
                frequency: _,
            } => {
                len += 1;
                left.traverse(len, codes);
                right.traverse(len, codes);
            }
            HuffmanNode::Leaf { byte, fequency: _ } => {
                codes.push(CodeLength { byte: *byte, len });
            }
        }
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency().eq(&other.frequency())
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.frequency().cmp(&self.frequency())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_frequency() {
        let leaf = HuffmanNode::Leaf {
            byte: 42,
            fequency: 10,
        };
        assert_eq!(leaf.frequency(), 10);
    }

    #[test]
    fn test_internal_frequency() {
        let left = HuffmanNode::Leaf {
            byte: 1,
            fequency: 5,
        };
        let right = HuffmanNode::Leaf {
            byte: 2,
            fequency: 15,
        };
        let internal = HuffmanNode::Internal {
            left: Box::new(left),
            right: Box::new(right),
            frequency: 20,
        };
        assert_eq!(internal.frequency(), 20);
    }

    #[test]
    fn test_from_array_creates_correct_leaves() {
        let mut freqs = Frequencies::new();
        freqs[10] = 3;
        freqs[20] = 5;

        let root = HuffmanNode::from_frequencies(&freqs);
        assert_eq!(root.frequency(), 8);

        fn find_leaf(node: &HuffmanNode, byte: u8) -> Option<u64> {
            match node {
                HuffmanNode::Leaf { byte: b, fequency } if *b == byte => Some(*fequency),
                HuffmanNode::Internal { left, right, .. } => {
                    find_leaf(left, byte).or_else(|| find_leaf(right, byte))
                }
                _ => None,
            }
        }

        assert_eq!(find_leaf(&root, 10), Some(3));
        assert_eq!(find_leaf(&root, 20), Some(5));
    }

    #[test]
    fn test_ordering_of_nodes() {
        let leaf_small = HuffmanNode::Leaf {
            byte: 0,
            fequency: 1,
        };
        let leaf_large = HuffmanNode::Leaf {
            byte: 1,
            fequency: 10,
        };

        let mut heap = BinaryHeap::new();
        heap.push(leaf_small);
        heap.push(leaf_large);

        let first = heap.pop().unwrap();
        assert_eq!(first.frequency(), 1);
        let second = heap.pop().unwrap();
        assert_eq!(second.frequency(), 10);
    }

    #[test]
    fn test_tree_combination() {
        let mut freqs = Frequencies::new();
        freqs[1] = 2;
        freqs[2] = 3;
        freqs[3] = 5;

        let root = HuffmanNode::from_frequencies(&freqs);
        assert_eq!(root.frequency(), 10);

        fn has_internal(node: &HuffmanNode) -> bool {
            match node {
                HuffmanNode::Internal { .. } => true,
                HuffmanNode::Leaf { .. } => false,
            }
        }

        assert!(has_internal(&root));
    }
}
