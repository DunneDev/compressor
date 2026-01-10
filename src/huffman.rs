mod bit_writer;
mod byte_map;
mod encoder;
mod frequency;
mod tree;

use crate::huffman::bit_writer::BitWriter;
use frequency::Frequencies;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, SeekFrom};

const BYTE_ALPHABET_SIZE: usize = 256;
const BUFFER_SIZE: usize = 8192;

pub fn compress<R, W>(input: R, output: W) -> io::Result<()>
where
    R: Read + Seek,
    W: Write,
{
    let mut reader = BufReader::new(input);
    let mut writer = BitWriter::new(BufWriter::new(output));

    let byte_map = Frequencies::from_input(&mut reader)?
        .to_huff_tree()
        .to_byte_map();

    let pos = reader.seek(SeekFrom::Start(0))?;

    let mut test = [0u8; 1];
    let n = reader.read(&mut test)?;
    println!("read {} byte(s)", n);
    assert_eq!(pos, 0);
    byte_map.encode(&mut reader, &mut writer)
}
