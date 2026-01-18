use crate::huffman::BYTE_ALPHABET_SIZE;
use crate::huffman::tree::HuffmanNode;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Frequencies([u64; BYTE_ALPHABET_SIZE]);

impl Frequencies {
    pub fn new() -> Self {
        Frequencies([0; BYTE_ALPHABET_SIZE])
    }

    pub fn from_input<R>(reader: &mut BufReader<R>) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let mut frequencies = Frequencies::new();

        loop {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                break;
            }

            for &byte in buffer {
                frequencies.count_byte(byte);
            }

            let length = buffer.len();
            reader.consume(length);
        }

        Ok(frequencies)
    }

    pub fn to_huff_tree(&self) -> HuffmanNode {
        HuffmanNode::from_frequencies(self)
    }

    fn count_byte(&mut self, byte: u8) {
        self[byte as usize] += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.iter().all(|&freq| freq == 0)
    }
}

impl Deref for Frequencies {
    type Target = [u64; BYTE_ALPHABET_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Frequencies {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn get_frequencies_empty_input() {
        let mut input = BufReader::new(Cursor::new(""));
        let output = Frequencies::from_input(&mut input).unwrap();
        let expected = Frequencies::new();

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_single_byte() {
        let mut input = BufReader::new(Cursor::new([42u8]));
        let output = Frequencies::from_input(&mut input).unwrap();

        let mut expected = Frequencies::new();
        expected[42] = 1;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_all_bytes_once() {
        let mut input: BufReader<Cursor<Vec<u8>>> =
            BufReader::new(Cursor::new((0u8..=255u8).collect()));
        let output = Frequencies::from_input(&mut input).unwrap();

        let mut expected = Frequencies::new();
        for count in expected.iter_mut() {
            *count = 1;
        }

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_repeated_pattern() {
        let mut input = BufReader::new(Cursor::new(b"abcabcabcabc"));
        let output = Frequencies::from_input(&mut input).unwrap();

        let mut expected = Frequencies::new();
        expected[b'a' as usize] = 4;
        expected[b'b' as usize] = 4;
        expected[b'c' as usize] = 4;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_large_input() {
        let mut input = BufReader::new(Cursor::new(vec![b'x'; 20_000]));
        let output = Frequencies::from_input(&mut input).unwrap();

        let mut expected = Frequencies::new();
        expected[b'x' as usize] = 20_000;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_binary_data() {
        let mut input = BufReader::new(Cursor::new([0, 255, 0, 128, 255, 128]));
        let output = Frequencies::from_input(&mut input).unwrap();

        let mut expected = Frequencies::new();
        expected[0] = 2;
        expected[128] = 2;
        expected[255] = 2;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_whitespace_and_newlines() {
        let mut input = BufReader::new(Cursor::new(b" \n\t\n "));
        let output = Frequencies::from_input(&mut input).unwrap();

        let mut expected = Frequencies::new();
        expected[b' ' as usize] = 2;
        expected[b'\n' as usize] = 2;
        expected[b'\t' as usize] = 1;

        assert_eq!(output, expected);
    }
}
