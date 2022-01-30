use std::io::{BufWriter, Error, Write};

#[repr(transparent)]
pub struct UwUOutFile<T: Write> {
    writer: BufWriter<T>,
}

impl<T: Write> UwUOutFile<T> {
    #[inline]
    pub fn new(writer: T) -> UwUOutFile<T> {
        UwUOutFile {
            writer: BufWriter::new(writer),
        }
    }

    #[inline]
    pub fn write_bytes(&mut self, write_bytes: &[u8]) -> Result<(), Error> {
        self.writer.write_all(write_bytes)
    }
}
