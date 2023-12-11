extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::error::DBError;
use crate::traits::FileTrait;
use crate::BLOCK_SIZE;
use crate::EOE_BLOCK;

pub struct Range<const N: usize> {
    start: usize,
    end: usize,
}

impl<const N: usize> Range<N> {
    pub fn new() -> Self {
        let start: usize = 0;
        let end: usize = 0;
        Self { start, end }
    }

    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }
}

pub struct DBStreamCache<const N: usize> {
    ready: bool,
    offset: usize,
    range: Range<N>,
    cache: [u8; N],
}

impl<const N: usize> DBStreamCache<N> {
    fn get_segment_bounds(&self) -> (usize, usize) {
        let head: usize = self.offset;
        let tail: usize = head + BLOCK_SIZE;
        (head, tail)
    }

    fn update_offset(&mut self, tail: usize) {
        if tail == N {
            self.ready = false;
            self.offset += BLOCK_SIZE;
        } else {
            self.offset += BLOCK_SIZE;
        }
    }
}

impl<const N: usize> DBStreamCache<N> {
    pub fn new() -> Self {
        let ready: bool = false;
        let offset: usize = 0;
        let range: Range<N> = Range::new();
        let cache: [u8; N] = [0; N];
        Self {
            ready,
            offset,
            range,
            cache,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn set_ready(&mut self, status: bool) {
        self.ready = status;
    }

    pub fn set_cache(&mut self, cache: [u8; N], start: usize, end: usize) {
        self.ready = true;
        self.offset = 0;
        self.cache = cache;

        self.range.set_start(start);
        self.range.set_end(end);
    }

    pub fn get_chunk(&mut self) -> [u8; BLOCK_SIZE] {
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

    pub fn seek_from_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

pub struct DBFileStream<'a, const N: usize> {
    file: &'a mut Box<dyn FileTrait>,
    cache: DBStreamCache<N>,
    file_position: usize,
}

impl<'a, const N: usize> DBFileStream<'a, N> {
    fn update_cache(&mut self) -> Result<(), DBError> {
        if !self.cache.is_ready() {
            let start: usize = self.file_position;
            let end: usize = start + N;

            let mut buffer: [u8; N] = [0; N];
            let result: Result<(), DBError> = self.file.read_exact(&mut buffer);
            self.file_position += N;

            if let Err(error) = result {
                // This assumes that the read buffer contains EOE at the end
                // Which is not ideal.
                let mut unused_length: usize = 0;
                for value in buffer.iter().rev() {
                    if *value == 0 {
                        unused_length += 1;
                    } else {
                        break;
                    }
                }
                self.file_position -= unused_length;

                if unused_length == N {
                    return Err(error);
                }
            }
            self.cache.set_cache(buffer, start, end);
            return Ok(());
        }
        Ok(())
    }

    fn seek_from_start(&mut self, start: usize) -> Result<(), DBError> {
        let cache_st: usize = self.cache.range.start;
        let cache_en: usize = self.cache.range.end;

        if cache_st <= start && cache_en > start {
            let offset: usize = start - cache_st;
            self.cache.seek_from_offset(offset);
        }

        let seek: usize = self.file.seek(start)?;
        self.file_position = seek;
        Ok(())
    }

    fn write(&mut self, buffer: &[u8]) -> Result<(), DBError> {
        let write_len: usize = self.file.write(buffer).unwrap();
        self.file_position += write_len;
        Ok(())
    }

    fn rebuild_database(
        &mut self,
        st_pos1: &mut usize,
        en_pos1: &mut usize,
    ) -> Result<(), DBError> {
        loop {
            let current_chunk: Vec<u8> = self.next_chunk()?;
            let current_uid: &[u8] = &current_chunk[..BLOCK_SIZE];

            // Get next chunk bounds
            let (st_pos2, en_pos2) = self.get_chunk_bounds()?;

            // Get next chunk
            let mut next_chunk: Vec<u8> = self.next_chunk()?;

            // Overwrite UID from current chunk
            next_chunk[..BLOCK_SIZE].copy_from_slice(current_uid);

            // Seek to current chunk start
            self.seek_from_start(*st_pos1)?;

            // Overwrite next chunk data
            self.write(&next_chunk)?;

            // Seek to the end of current chunk
            self.seek_from_start(*en_pos1)?;

            *st_pos1 = st_pos2;
            *en_pos1 = en_pos2;
        }
    }
}

impl<'a, const N: usize> DBFileStream<'a, N> {
    pub fn new(file: &'a mut Box<dyn FileTrait>) -> Self {
        let cache: DBStreamCache<N> = DBStreamCache::new();
        let file_position: usize = 0;
        DBFileStream {
            file,
            cache,
            file_position,
        }
    }

    pub fn get_chunk_bounds(&mut self) -> Result<(usize, usize), DBError> {
        self.update_cache()?;
        let cache_st: usize = self.cache.offset + self.cache.range.start;
        for block in self.into_iter() {
            if let Ok(block) = block {
                if block == EOE_BLOCK {
                    break;
                }
                continue;
            }
            return Err(DBError::InvalidData);
        }

        let cache_en: usize = self.cache.offset + self.cache.range.start;
        self.seek_from_start(cache_st)?;
        Ok((cache_st, cache_en))
    }

    pub fn append_end(&mut self, data: &[u8]) {
        while let Ok(_) = self.next_chunk() {}
        let write_len: usize = self.file.write(data).unwrap();
        self.file_position += write_len;
    }

    pub fn last_chunk(&mut self) -> Option<Vec<u8>> {
        let mut last_chunk: Option<Vec<u8>> = None;
        while let Ok(chunk) = self.next_chunk() {
            last_chunk = Some(chunk);
        }
        last_chunk
    }

    pub fn next_chunk(&mut self) -> Result<Vec<u8>, DBError> {
        let mut data: Vec<u8> = Vec::new();

        for block in self.into_iter() {
            if let Ok(block) = block {
                data.extend(block);
                if block == EOE_BLOCK {
                    return Ok(data);
                }
                continue;
            }
            return Err(DBError::InvalidData);
        }
        Err(DBError::InvalidData)
    }

    pub fn remove_chunk(&mut self) -> Result<(), DBError> {
        let (mut st_pos1, mut en_pos1) = self.get_chunk_bounds()?;
        self.rebuild_database(&mut st_pos1, &mut en_pos1)?;
        self.file.set_len(st_pos1)?;
        Ok(())
    }
}

impl<'a, const N: usize> Iterator for DBFileStream<'a, N> {
    type Item = Result<[u8; BLOCK_SIZE], DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_ready() {
            let bytes: [u8; BLOCK_SIZE] = self.cache.get_chunk();
            return Some(Ok(bytes));
        }

        let result: Result<(), DBError> = self.update_cache();
        if let Err(error) = result {
            return Some(Err(error));
        }

        let bytes: [u8; BLOCK_SIZE] = self.cache.get_chunk();
        return Some(Ok(bytes));
    }
}
