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

pub fn compress<R, W>(input: R, output: W) -> io::Result<()>
where
    R: Read + Seek,
    W: Write,
{
    let mut reader = BufReader::new(input);
    let mut writer = BitWriter::new(BufWriter::new(output));

    let frequencies = Frequencies::from_input(&mut reader)?;

    if frequencies.is_empty() {
        return Ok(());
    }

    reader.seek(SeekFrom::Start(0))?;

    frequencies
        .to_huff_tree()
        .to_byte_map()
        .encode(&mut reader, &mut writer)
}

#[cfg(test)]
mod tests {
    use super::compress;
    use std::io::Cursor;

    fn compress_bytes(input: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        compress(Cursor::new(input), &mut output).expect("compression should succeed");
        output
    }

    #[test]
    fn compress_empty_input() {
        let output = compress_bytes(b"");

        assert!(output.is_empty());
    }

    #[test]
    fn compress_single_byte_input() {
        let output = compress_bytes(b"a");

        let expected = vec![
            0,
            1, // number of codes
            b'a',
            1,           // code table entry
            0b1000_0000, // encoded data (padded)
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn compress_single_repeated_symbol() {
        let output = compress_bytes(b"aaaa");

        let expected = vec![
            0,
            1, // number of codes
            b'a',
            1,           // code table entry
            0b1111_0000, // 4 bits of '1' padded
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn compress_two_symbols_equal_frequency() {
        let output = compress_bytes(b"abab");

        let expected = vec![
            0,
            2, // number of codes
            b'a',
            1,
            b'b',
            1,
            0b0101_0000,
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn compress_is_deterministic() {
        let input = b"the quick brown fox jumps over the lazy dog";

        let out1 = compress_bytes(input);
        let out2 = compress_bytes(input);

        assert_eq!(out1, out2, "compression must be deterministic");
    }

    #[test]
    fn compress_many_symbols_skewed_distribution() {
        let input = b"aaaaaaaaaabbbccd";

        let output = compress_bytes(input);

        assert!(output.len() > 3, "output should contain header + data");

        let num_codes = u16::from_be_bytes([output[0], output[1]]) as usize;

        let header_size = 1 + num_codes * 2;

        assert!(
            output.len() >= header_size,
            "output should contain a full header"
        );

        let mut prev_len = 0;
        for i in 0..num_codes {
            let len = output[1 + i * 2 + 1];
            assert!(
                len >= prev_len,
                "code lengths must be sorted in non-decreasing order"
            );
            prev_len = len;
        }
    }

    #[test]
    fn compress_all_unique_bytes() {
        let input: Vec<u8> = (0u8..=255u8).collect();
        let output = compress_bytes(&input);

        assert!(!output.is_empty(), "output should not be empty");

        let num_codes = u16::from_be_bytes([output[0], output[1]]) as usize;
        assert_eq!(num_codes, 256, "all unique bytes should produce 256 codes");
    }
}
