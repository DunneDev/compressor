use crate::huffman::bit_writer::BitWriter;
use crate::huffman::byte_map::ByteMap;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};

pub struct Encoder<'a, R, W>
where
    R: Read,
    W: Write,
{
    reader: &'a mut BufReader<R>,
    writer: &'a mut BitWriter<BufWriter<W>>,
    byte_map: &'a ByteMap,
}

impl<'a, R, W> Encoder<'a, R, W>
where
    R: Read,
    W: Write + std::fmt::Debug,
{
    pub fn new(
        reader: &'a mut BufReader<R>,
        writer: &'a mut BitWriter<BufWriter<W>>,
        byte_map: &'a ByteMap,
    ) -> Self {
        Encoder {
            reader,
            writer,
            byte_map,
        }
    }

    pub fn encode(self) -> io::Result<()> {
        println!("{:?}", self.byte_map);
        self.encode_codes()?.encode_data()
    }

    fn encode_codes(self) -> io::Result<Self> {
        let mut codes: Vec<(u8, u8)> = self
            .byte_map
            .iter()
            .map(|(byte, code)| (*byte, code.len))
            .collect();

        codes
            .sort_by(|&(byte_a, len_a), (byte_b, len_b)| len_a.cmp(len_b).then(byte_a.cmp(byte_b)));

        self.writer.write_bytes(&[codes.len() as u8])?;

        for (byte, len) in codes {
            self.writer.write_bytes(&[byte, len])?;
        }

        Ok(self)
    }

    fn encode_data(self) -> io::Result<()> {
        loop {
            let buffer = self.reader.fill_buf()?;
            let length = buffer.len();

            if length == 0 {
                break;
            }

            for byte in buffer {
                let code = self
                    .byte_map
                    .get(byte)
                    .expect("Every byte should have a key if byte_map was constructed properly");

                self.writer.write_bits(code.bit_pattern, code.len)?;
            }

            self.reader.consume(length);
        }

        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::huffman::frequency::Frequencies;

    use super::*;

    fn create_input<T>(input: T) -> BufReader<Cursor<T>>
    where
        T: std::convert::AsRef<[u8]>,
    {
        BufReader::new(Cursor::new(input))
    }

    fn create_output() -> BitWriter<BufWriter<Cursor<Vec<u8>>>> {
        BitWriter::new(BufWriter::new(Cursor::new(vec![])))
    }

    #[test]
    fn encode_single_byte() -> std::io::Result<()> {
        let mut input = create_input(b"3");
        let mut output = create_output();

        let byte_map = Frequencies::from_input(&mut input)?
            .to_huff_tree()
            .to_byte_map();

        input.rewind()?;

        let encoder = Encoder::new(&mut input, &mut output, &byte_map);
        encoder.encode()?;

        let inner_vec = output.into_inner().into_inner()?.into_inner();

        let expected = [1, b'3', 1, 128];
        assert_eq!(inner_vec, expected);

        Ok(())
    }

    #[test]
    fn encode_single_symbol_many_bytes() -> std::io::Result<()> {
        let mut input = create_input([50; 64]);
        let mut output = create_output();

        let byte_map = Frequencies::from_input(&mut input)?
            .to_huff_tree()
            .to_byte_map();

        input.rewind()?;

        byte_map.encode(&mut input, &mut output)?;
        output.flush()?;

        let inner_vec = output.into_inner().into_inner()?.into_inner();

        let mut expected = [255; 11];
        expected[0] = 1;
        expected[1] = 50;
        expected[2] = 1;

        assert_eq!(inner_vec, expected);

        Ok(())
    }

    #[test]
    fn encode_alternating_symbols() -> std::io::Result<()> {
        let mut input = create_input(b"abababab");
        let mut output = create_output();

        let byte_map = Frequencies::from_input(&mut input)?
            .to_huff_tree()
            .to_byte_map();

        input.rewind()?;

        byte_map.encode(&mut input, &mut output)?;
        output.flush()?;

        let inner_vec = output.into_inner().into_inner()?.into_inner();

        let expected = [2, 97, 1, 98, 1, 85];

        assert_eq!(inner_vec, expected);

        Ok(())
    }

    #[test]
    fn encode_varying_frequencies() -> std::io::Result<()> {
        let mut input = create_input(b"aaaabc");
        let mut output = create_output();

        let byte_map = Frequencies::from_input(&mut input)?
            .to_huff_tree()
            .to_byte_map();

        input.rewind()?;

        byte_map.encode(&mut input, &mut output)?;
        output.flush()?;

        let inner_vec = output.into_inner().into_inner()?.into_inner();

        let expected = [3, 97, 1, 98, 2, 99, 2, 11];

        assert_eq!(inner_vec, expected);

        Ok(())
    }
}
