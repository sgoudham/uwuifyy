use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::Path;

pub struct UwUInFile {
    file_bytes: u64,
    reader: BufReader<File>,
    pub buffer: String,
}

#[repr(transparent)]
pub struct UwUOutFile<T: Write> {
    writer: BufWriter<T>,
}

impl UwUInFile {
    #[inline]
    pub fn new(path: &Path) -> Result<UwUInFile, Error> {
        let file = File::open(path)?;

        Ok(UwUInFile {
            file_bytes: file.metadata()?.len(),
            reader: BufReader::new(file),
            buffer: String::new(),
        })
    }

    #[inline]
    pub fn read_until_newline(&mut self) -> Result<usize, Error> {
        self.reader.read_line(&mut self.buffer)
    }

    #[inline]
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    #[inline]
    pub fn get_file_bytes(&self) -> u64 {
        self.file_bytes
    }
}

impl<T: Write> UwUOutFile<T> {
    #[inline]
    pub fn new(writer: T) -> UwUOutFile<T> {
        UwUOutFile {
            writer: BufWriter::new(writer),
        }
    }

    #[inline]
    pub fn write_newline(&mut self) -> Result<(), Error> {
        self.writer.write_all(b"\n")
    }

    #[inline]
    pub fn write_string(&mut self, write_str: &str) -> Result<(), Error> {
        self.writer.write_all(write_str.as_bytes())
    }

    #[inline]
    pub fn write_bytes(&mut self, write_bytes: &[u8]) -> Result<(), Error> {
        self.writer.write_all(write_bytes)
    }

    #[inline]
    pub fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<(), Error> {
        self.writer.write_fmt(fmt)
    }
}
