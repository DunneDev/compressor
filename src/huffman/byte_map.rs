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
        W: Write,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_code_length_produces_len_1_code() {
        let mut codes = vec![CodeLength { byte: b'a', len: 5 }];

        let map = ByteMap::new(&mut codes);

        let entry = map.get(&b'a').expect("entry should exist");
        assert_eq!(entry.len, 1);
        assert_eq!(entry.bit_pattern, 1);
    }

    #[test]
    fn canonical_codes_with_same_length_are_sequential() {
        let mut codes = vec![
            CodeLength { byte: b'a', len: 2 },
            CodeLength { byte: b'b', len: 2 },
            CodeLength { byte: b'c', len: 2 },
        ];

        let map = ByteMap::new(&mut codes);

        assert_eq!(map[&b'a'].bit_pattern, 0b00);
        assert_eq!(map[&b'b'].bit_pattern, 0b01);
        assert_eq!(map[&b'c'].bit_pattern, 0b10);

        assert_eq!(map[&b'a'].len, 2);
        assert_eq!(map[&b'b'].len, 2);
        assert_eq!(map[&b'c'].len, 2);
    }

    #[test]
    fn codes_are_sorted_by_length_then_byte() {
        let mut codes = vec![
            CodeLength { byte: b'c', len: 3 },
            CodeLength { byte: b'a', len: 2 },
            CodeLength { byte: b'b', len: 2 },
        ];

        let map = ByteMap::new(&mut codes);

        assert_eq!(map[&b'a'].bit_pattern, 0b00);
        assert_eq!(map[&b'b'].bit_pattern, 0b01);

        assert_eq!(map[&b'c'].bit_pattern, 0b100);
        assert_eq!(map[&b'c'].len, 3);
    }

    #[test]
    fn bit_pattern_shifts_when_length_increases() {
        let mut codes = vec![
            CodeLength { byte: b'a', len: 1 },
            CodeLength { byte: b'b', len: 3 },
        ];

        let map = ByteMap::new(&mut codes);

        assert_eq!(map[&b'a'].bit_pattern, 0b0);
        assert_eq!(map[&b'a'].len, 1);

        assert_eq!(map[&b'b'].bit_pattern, 0b100);
        assert_eq!(map[&b'b'].len, 3);
    }
}
