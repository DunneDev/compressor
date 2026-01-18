mod huffman;

use std::io::{self, prelude::*};

pub fn compress<R, W>(input: R, output: W) -> io::Result<()>
where
    R: Read + Seek,
    W: Write,
{
    huffman::compress(input, output)
}
