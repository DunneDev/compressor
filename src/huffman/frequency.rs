use super::BYTE_ALPHABET_SIZE;
use std::io::{self, BufReader, Read};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Frequencies([u64; BYTE_ALPHABET_SIZE]);

impl Frequencies {
    pub fn new() -> Self {
        Frequencies([0; BYTE_ALPHABET_SIZE])
    }

    pub fn from_input(input: impl Read) -> io::Result<Self> {
        const BUFFER_SIZE: usize = 8192;
        let mut buffer = [0; BUFFER_SIZE];
        let mut reader = BufReader::new(input);
        let mut frequencies = Frequencies([0; BYTE_ALPHABET_SIZE]);

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            for &byte in &buffer[..bytes_read] {
                frequencies.count_byte(byte);
            }
        }

        Ok(frequencies)
    }

    fn count_byte(&mut self, byte: u8) {
        self[byte as usize] += 1;
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

    // TODO: Remove
    // fn compare_output(output: &Frequencies, expected: &Frequencies) {
    //     for i in 0..BYTE_ALPHABET_SIZE {
    //         assert_eq!(output[i], expected[i]);
    //     }
    // }

    #[test]
    fn get_frequencies_empty_input() {
        let input = "".as_bytes();
        let output = Frequencies::from_input(input).unwrap();
        let expected = Frequencies::new();

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_single_byte() {
        let input = [42u8];
        let output = Frequencies::from_input(&input[..]).unwrap();

        let mut expected = Frequencies::new();
        expected[42] = 1;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_all_bytes_once() {
        let input: Vec<u8> = (0u8..=255u8).collect();
        let output = Frequencies::from_input(&input[..]).unwrap();

        let mut expected = Frequencies::new();
        for count in expected.iter_mut() {
            *count = 1;
        }

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_repeated_pattern() {
        let input = b"abcabcabcabc";
        let output = Frequencies::from_input(&input[..]).unwrap();

        let mut expected = Frequencies::new();
        expected[b'a' as usize] = 4;
        expected[b'b' as usize] = 4;
        expected[b'c' as usize] = 4;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_large_input() {
        let input = vec![b'x'; 20_000];
        let output = Frequencies::from_input(&input[..]).unwrap();

        let mut expected = Frequencies::new();
        expected[b'x' as usize] = 20_000;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_binary_data() {
        let input = [0, 255, 0, 128, 255, 128];
        let output = Frequencies::from_input(&input[..]).unwrap();

        let mut expected = Frequencies::new();
        expected[0] = 2;
        expected[128] = 2;
        expected[255] = 2;

        assert_eq!(output, expected);
    }

    #[test]
    fn get_frequencies_whitespace_and_newlines() {
        let input = b" \n\t\n ";
        let output = Frequencies::from_input(&input[..]).unwrap();

        let mut expected = Frequencies::new();
        expected[b' ' as usize] = 2;
        expected[b'\n' as usize] = 2;
        expected[b'\t' as usize] = 1;

        assert_eq!(output, expected);
    }
}
