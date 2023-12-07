use crate::BLOCK_SIZE;
use crate::EOT_BLOCK;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct DBFileStream {
    file: File,
}

impl DBFileStream {
    pub fn new(file: File) -> Self {
        DBFileStream { file }
    }

    pub fn append_end(&mut self, data: &[u8]) {
        while let Ok(_) = self.next_chunk() {}
        self.file.write_all(data).unwrap();
    }

    pub fn last_chunk(&mut self) -> Option<Vec<u8>> {
        let mut last_chunk: Option<Vec<u8>> = None;
        while let Ok(chunk) = self.next_chunk() {
            last_chunk = Some(chunk);
        }
        last_chunk
    }

    pub fn next_chunk(&mut self) -> Result<Vec<u8>, ()> {
        let mut data: Vec<u8> = Vec::new();

        for block in self.into_iter() {
            if let Ok(block) = block {
                data.extend(block);
                if block == EOT_BLOCK {
                    return Ok(data);
                }
                continue;
            }
            return Err(());
        }
        Err(())
    }
}

impl Iterator for DBFileStream {
    type Item = Result<[u8; BLOCK_SIZE], io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
        match self.file.read_exact(&mut buffer) {
            Ok(()) => Some(Ok(buffer)),
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}
