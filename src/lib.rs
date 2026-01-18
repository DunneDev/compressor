mod huffman;

use std::io::{self, prelude::*};

pub fn compress<R, W>(input: R, output: W) -> io::Result<()>
where
    R: Read + Seek,
    W: Write,
{
    huffman::compress(input, output)
}

pub fn decompress<R, W>(_input: R, _output: W) -> io::Result<()>
where
    R: Read + Seek,
    W: Write,
{
    todo!()
}
