use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::Path;

pub struct UwUInFile {
    file_bytes: u64,
    reader: BufReader<File>,
    buffer: Vec<u8>,
}

pub struct UwUOutFile {
    writer: BufWriter<File>,
}

impl UwUInFile {
    pub fn new(path: &Path) -> Result<UwUInFile, Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let file_metadata = match file.metadata() {
            Ok(file_metadata) => file_metadata,
            Err(err) => return Err(err),
        };
        let file_bytes = file_metadata.len();
        let reader = BufReader::new(file);
        let buffer = Vec::new();

        Ok(UwUInFile {
            file_bytes,
            reader,
            buffer,
        })
    }

    pub fn read_until_newline(&mut self) -> Result<usize, Error> {
        match self.reader.read_until(b'\n', &mut self.buffer) {
            Ok(byte_vec) => Ok(byte_vec),
            Err(err) => Err(err),
        }
    }

    pub fn get_buffer_as_utf8_str(&self) -> String {
        String::from_utf8_lossy(&self.buffer).to_string()
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn get_file_bytes(&self) -> u64 {
        self.file_bytes
    }
}

impl UwUOutFile {
    pub fn new(path: &str) -> Result<UwUOutFile, Error> {
        let file = match File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let writer = BufWriter::new(file);

        Ok(UwUOutFile { writer })
    }

    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn write_string_with_newline(&mut self, write_str: &str) -> Result<(), Error> {
        match self.writer.write_all(format!("{}\n", write_str).as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn write_string(&mut self, write_str: &str) -> Result<(), Error> {
        match self.writer.write_all(write_str.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
