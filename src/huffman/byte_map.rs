use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::ops;

use crate::huffman::bit_writer::BitWriter;
use crate::huffman::encoder::Encoder;

pub struct CodeLength {
    pub byte: u8,
    pub len: u8,
}

#[derive(Debug)]
pub struct CodeEntry {
    pub bit_pattern: u32,
    pub len: u8,
}

#[derive(Debug)]
pub struct ByteMap(HashMap<u8, CodeEntry>);

impl ByteMap {
    pub fn new(code_lengths: &mut [CodeLength]) -> Self {
        code_lengths.sort_by(|a, b| a.len.cmp(&b.len).then(a.byte.cmp(&b.byte)));

        let mut byte_map = ByteMap(HashMap::new());

        if code_lengths.len() == 1 {
            let code_len = code_lengths.first().unwrap();
            byte_map.insert(
                code_len.byte,
                CodeEntry {
                    bit_pattern: 1,
                    len: 1,
                },
            );

            return byte_map;
        }

        let mut bit_pattern = 0;
        let mut prev_len = 0;

        for code in code_lengths {
            if code.len > prev_len {
                bit_pattern <<= code.len - prev_len
            }

            byte_map.insert(
                code.byte,
                CodeEntry {
                    bit_pattern,
                    len: code.len,
                },
            );

            bit_pattern += 1;
            prev_len = code.len;
        }

        byte_map
    }
}

impl ByteMap {
    pub fn encode<R, W>(
        &self,
        reader: &mut BufReader<R>,
        output: &mut BitWriter<BufWriter<W>>,
    ) -> io::Result<()>
    where
        R: Read + Seek,
        W: Write + std::fmt::Debug,
    {
        let encoder = Encoder::new(reader, output, self);
        encoder.encode()
    }
}

impl ops::Deref for ByteMap {
    type Target = HashMap<u8, CodeEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ByteMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
