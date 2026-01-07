use std::io::{self, BufReader, Read};

const BYTE_SIZE: usize = 256;

pub fn get_frequencies(input: impl Read) -> io::Result<Box<[u64; BYTE_SIZE]>> {
    let mut reader = BufReader::new(input);
    const BUFFER_SIZE: usize = 8192;
    let mut buffer = [0; BUFFER_SIZE];
    let mut frequencies = [0; BYTE_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for &byte in &buffer[..bytes_read] {
            frequencies[byte as usize] += 1;
        }
    }

    Ok(Box::new(frequencies))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_output(output: &[u64; BYTE_SIZE], expected: &[u64; BYTE_SIZE]) {
        for i in 0..BYTE_SIZE {
            assert_eq!(output[i], expected[i]);
        }
    }

    #[test]
    fn get_frequencies_empty_input() {
        let input = "".as_bytes();
        let output = get_frequencies(input).unwrap();
        let expected = Box::new([0; BYTE_SIZE]);

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_simple_input() {
        let input = "Hello".as_bytes();
        let output = get_frequencies(input).unwrap();
        let mut expected = Box::new([0; BYTE_SIZE]);

        expected[b'H' as usize] = 1;
        expected[b'e' as usize] = 1;
        expected[b'l' as usize] = 2;
        expected[b'o' as usize] = 1;

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_single_byte() {
        let input = [42u8];
        let output = get_frequencies(&input[..]).unwrap();

        let mut expected = [0u64; BYTE_SIZE];
        expected[42] = 1;

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_all_bytes_once() {
        let input: Vec<u8> = (0u8..=255u8).collect();
        let output = get_frequencies(&input[..]).unwrap();

        let mut expected = [0u64; BYTE_SIZE];
        for count in expected.iter_mut() {
            *count = 1;
        }

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_repeated_pattern() {
        let input = b"abcabcabcabc";
        let output = get_frequencies(&input[..]).unwrap();

        let mut expected = [0u64; BYTE_SIZE];
        expected[b'a' as usize] = 4;
        expected[b'b' as usize] = 4;
        expected[b'c' as usize] = 4;

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_large_input() {
        let input = vec![b'x'; 20_000];
        let output = get_frequencies(&input[..]).unwrap();

        let mut expected = [0u64; BYTE_SIZE];
        expected[b'x' as usize] = 20_000;

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_binary_data() {
        let input = [0, 255, 0, 128, 255, 128];
        let output = get_frequencies(&input[..]).unwrap();

        let mut expected = [0u64; BYTE_SIZE];
        expected[0] = 2;
        expected[128] = 2;
        expected[255] = 2;

        compare_output(&output, &expected);
    }

    #[test]
    fn get_frequencies_whitespace_and_newlines() {
        let input = b" \n\t\n ";
        let output = get_frequencies(&input[..]).unwrap();

        let mut expected = [0u64; BYTE_SIZE];
        expected[b' ' as usize] = 2;
        expected[b'\n' as usize] = 2;
        expected[b'\t' as usize] = 1;

        compare_output(&output, &expected);
    }
}
