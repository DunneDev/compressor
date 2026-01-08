use crate::huffman::frequency::Frequencies;
use crate::huffman::tree::HuffmanNode;
use bimap::BiMap;

#[derive(Debug)]
pub struct ByteMap(BiMap<u8, Vec<bool>>);

impl From<HuffmanNode> for ByteMap {
    fn from(value: HuffmanNode) -> Self {
        ByteMap(BiMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn byte_map_from_huffman() {
        let mut counts = Frequencies::new();
        counts[b'a' as usize] = 5;
        counts[b'b' as usize] = 9;
        counts[b'c' as usize] = 12;
        counts[b'd' as usize] = 13;
        counts[b'e' as usize] = 16;
        counts[b'f' as usize] = 45;

        let tree = HuffmanNode::from_frequencies(&counts);
        let map = ByteMap::from(tree);

        let mut expected: BiMap<u8, Vec<bool>> = BiMap::new();
        expected.insert(b'f', vec![false]);
        expected.insert(b'c', vec![true, false, false]);
        expected.insert(b'd', vec![true, false, true]);
        expected.insert(b'a', vec![true, true, false, false]);
        expected.insert(b'b', vec![true, true, false, true]);
        expected.insert(b'e', vec![true, true, true]);

        assert_eq!(map.0, expected);
    }
}
