use crate::huffman::BYTE_ALPHABET_SIZE;
use crate::huffman::byte_map::{ByteMap, CodeEntry};
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

    pub fn to_byte_map(&self) -> ByteMap {
        let mut byte_map = ByteMap::new();
        self.traverse(0, 0, &mut byte_map);
        byte_map
    }

    fn frequency(&self) -> u64 {
        match self {
            HuffmanNode::Leaf { fequency, .. } => *fequency,
            HuffmanNode::Internal { frequency, .. } => *frequency,
        }
    }

    fn traverse(&self, bits: u32, mut length: u8, byte_map: &mut HashMap<u8, CodeEntry>) {
        match self {
            HuffmanNode::Internal {
                left,
                right,
                frequency: _,
            } => {
                length += 1;
                left.traverse(bits << 1, length, byte_map);
                right.traverse((bits + 1) << 1, length, byte_map);
            }
            HuffmanNode::Leaf { byte, fequency: _ } => {
                byte_map.insert(*byte, CodeEntry { bits, length });
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

    // #[test]
    // fn tree_to_byte_map() {
    //     let mut freq = Frequencies::new();
    //     freq[b'a' as usize] = 5;
    //     freq[b'b' as usize] = 9;
    //     freq[b'c' as usize] = 12;
    //     freq[b'd' as usize] = 13;
    //     freq[b'e' as usize] = 16;
    //     freq[b'f' as usize] = 45;
    //
    //     let map = freq.to_huff_tree().to_byte_map();
    //
    //     let mut expected = ByteMap::new();
    //     expected.insert(b'f', vec![false]);
    //     expected.insert(b'c', vec![true, false, false]);
    //     expected.insert(b'd', vec![true, false, true]);
    //     expected.insert(b'a', vec![true, true, false, false]);
    //     expected.insert(b'b', vec![true, true, false, true]);
    //     expected.insert(b'e', vec![true, true, true]);
    //
    //     assert_eq!(map, expected);
    // }
}
