use std::io::{self, Write};

const U8_BITS: u8 = u8::BITS as u8;

pub struct BitWriter<T: Write> {
    writer: T,
    byte_buffer: u8,
    bits_filled: u8,
}

impl<T: Write> BitWriter<T> {
    pub fn new(writer: T) -> Self {
        BitWriter {
            writer,

            byte_buffer: 0,
            bits_filled: 0,
        }
    }

    pub fn into_inner(self) -> T {
        self.writer
    }

    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.byte_buffer = (self.byte_buffer << 1) | bit as u8;
        self.bits_filled += 1;

        if self.bits_filled == U8_BITS {
            self.writer.write_all(&[self.byte_buffer])?;
            self.byte_buffer = 0;
            self.bits_filled = 0;
        }

        Ok(())
    }

    pub fn write_bits(&mut self, bits: u32, length: u8) -> io::Result<()> {
        if length == 0 {
            return Ok(());
        }

        for i in 1..=length {
            self.write_bit((bits >> (length - i)) & 1 == 1)?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.bits_filled > 0 {
            self.byte_buffer <<= U8_BITS - self.bits_filled;
            self.writer.write_all(&[self.byte_buffer])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn write_single_bit() {
        let mut writer = BitWriter::new(io::Cursor::new(vec![]));
        writer.write_bit(true).unwrap();
        writer.flush().unwrap();
        let expected = vec![128];

        assert_eq!(writer.writer.into_inner(), expected);
    }

    #[test]
    fn write_true_and_false() {
        let mut writer = BitWriter::new(io::Cursor::new(vec![]));
        writer.write_bit(false).unwrap();
        writer.write_bit(true).unwrap();
        writer.flush().unwrap();
        let expected = vec![64];

        assert_eq!(writer.writer.into_inner(), expected);
    }

    #[test]
    fn write_bit_overflow_byte() {
        let mut writer = BitWriter::new(io::Cursor::new(vec![]));
        for _ in 0..9 {
            writer.write_bit(true).unwrap();
        }

        writer.flush().unwrap();
        let expected = vec![255, 128];

        assert_eq!(writer.writer.into_inner(), expected);
    }

    #[test]
    fn write_bits_overflow_byte() {
        let mut writer = BitWriter::new(io::Cursor::new(vec![]));
        writer.write_bits(256, 16).unwrap();
        writer.flush().unwrap();
        let expected = vec![1, 0];

        assert_eq!(writer.writer.into_inner(), expected);
    }
}
