mod frequency;
mod tree;

use frequency::get_frequencies;
use std::io::{self, Read, Write};
use tree::HuffmanNode;

const BYTE_SIZE: usize = 256;

pub fn compress(input: impl Read, output: impl Write) -> io::Result<()> {
    let freq = get_frequencies(input)?;

    let tree_head = HuffmanNode::from(&*freq);

    Ok(())
}
