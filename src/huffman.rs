mod byte_map;
mod frequency;
mod tree;

use byte_map::ByteMap;
use frequency::Frequencies;
use std::io::{self, Read, Write};
use tree::HuffmanNode;

const BYTE_ALPHABET_SIZE: usize = 256;

pub fn compress(input: impl Read, _output: impl Write) -> io::Result<()> {
    let freq = Frequencies::from_input(input)?;

    let tree_head = HuffmanNode::from_frequencies(&freq);

    let byte_map = ByteMap::from(tree_head);
    println!("{:?}", byte_map);

    Ok(())
}
