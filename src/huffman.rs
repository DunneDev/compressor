mod bit_writer;
mod byte_map;
mod frequency;
mod tree;

use frequency::Frequencies;
use std::io::{self, Read, Write};
use tree::HuffmanNode;

const BYTE_ALPHABET_SIZE: usize = 256;
const BUFFER_SIZE: usize = 8192;

pub fn compress(input: impl Read, _output: impl Write) -> io::Result<()> {
    let huffman_tree = Frequencies::from_input(input)?.to_huff_tree();

    Ok(())
}
