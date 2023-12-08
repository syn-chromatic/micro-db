use crate::BLOCK_SIZE;
use crate::EOE_BLOCK;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct DBStreamCache<const N: usize> {
    ready: bool,
    offset: usize,
    cache: [u8; N],
}

impl<const N: usize> DBStreamCache<N> {
    fn get_segment_bounds(&self) -> (usize, usize) {
        let head: usize = self.offset * BLOCK_SIZE;
        let tail: usize = head + BLOCK_SIZE;
        (head, tail)
    }

    fn update_offset(&mut self, tail: usize) {
        if tail == N {
            self.ready = false;
            self.offset = 0;
        } else {
            self.offset += 1;
        }
    }
}

impl<const N: usize> DBStreamCache<N> {
    pub fn new() -> Self {
        let ready: bool = false;
        let chunk_idx: usize = 0;
        let cache: [u8; N] = [0; N];
        Self {
            ready,
            offset: chunk_idx,
            cache,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn set_cache(&mut self, cache: [u8; N]) {
        self.cache = cache;
        self.ready = true;
    }

    pub fn get_next_chunk(&mut self) -> [u8; BLOCK_SIZE] {
        let (head, tail): (usize, usize) = self.get_segment_bounds();

        let bytes: &[u8] = &self.cache[head..tail];
        let chunk: [u8; BLOCK_SIZE] = {
            let mut result: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
            result.copy_from_slice(&bytes[..BLOCK_SIZE]);
            result
        };

        self.update_offset(tail);
        chunk
    }
}

pub struct DBFileStream<const N: usize> {
    file: File,
    cache: DBStreamCache<N>,
}

impl<const N: usize> DBFileStream<N> {
    pub fn new(file: File) -> Self {
        let cache: DBStreamCache<N> = DBStreamCache::new();
        DBFileStream { file, cache }
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
                if block == EOE_BLOCK {
                    return Ok(data);
                }
                continue;
            }
            return Err(());
        }
        Err(())
    }
}

impl<const N: usize> Iterator for DBFileStream<N> {
    type Item = Result<[u8; BLOCK_SIZE], io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_ready() {
            let bytes: [u8; 8] = self.cache.get_next_chunk();
            return Some(Ok(bytes));
        }

        let mut buffer: [u8; N] = [0; N];

        let result: Result<(), io::Error> = self.file.read_exact(&mut buffer);
        if let Err(error) = result {
            if buffer.iter().all(|&x| x == 0) {
                return Some(Err(error));
            }
        }

        self.cache.set_cache(buffer);
        let bytes: [u8; 8] = self.cache.get_next_chunk();
        return Some(Ok(bytes));
    }
}
