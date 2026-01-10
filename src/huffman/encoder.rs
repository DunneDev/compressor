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
    W: Write,
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
        loop {
            let buffer = self.reader.fill_buf()?;
            let length = buffer.len();

            if length == 0 {
                break Ok(());
            }

            for byte in buffer {
                let code = self
                    .byte_map
                    .get(byte)
                    .expect("Every byte should have a key if byte_map was constructed properly");

                self.writer.write_bits(code.bit_pattern, code.len)?
            }

            self.reader.consume(length);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::io::{Cursor, SeekFrom};

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

        output.flush()?;

        let inner_vec = output.into_inner().into_inner()?.into_inner();

        let expected = [128];
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

        let expected = [255; 8];

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

        let expected = [85];

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

        let expected = [11];

        assert_eq!(inner_vec, expected);

        Ok(())
    }
}
