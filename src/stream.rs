extern crate alloc;
use alloc::vec::Vec;

use crate::error::DBError;
use crate::serializer::UIDSerializer;
use crate::traits::FileBox;
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

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct DBStreamCache<'a, const N: usize> {
    file: &'a mut FileBox,
    position: usize,
    cache_offset: usize,
    cache_range: Range<N>,
    cache_buffer: [u8; N],
    cache_written: bool,
}

impl<'a, const N: usize> DBStreamCache<'a, N> {
    fn cache_position(&self) -> usize {
        self.cache_range.start + self.cache_offset
    }

    fn increment_cache_offset(&mut self) {
        if self.cache_offset < self.cache_range.end {
            self.cache_offset += BLOCK_SIZE;
        }
    }

    fn seek_cache_offset(&mut self, offset: usize) {
        self.cache_offset = offset;
    }

    fn set_cache(&mut self, cache: [u8; N], start: usize, end: usize) {
        self.cache_offset = 0;
        self.cache_buffer = cache;
        self.cache_written = false;

        self.cache_range.set_start(start);
        self.cache_range.set_end(end);
    }

    fn copy_into_cache(&mut self, buffer: &[u8]) {
        self.cache_buffer[self.cache_offset..self.cache_offset + buffer.len()]
            .copy_from_slice(buffer);

        self.cache_offset += buffer.len();
        self.cache_written = true;
    }

    fn flush_cache_buffer(&mut self) -> Result<(), DBError> {
        let start: usize = self.position;
        let cache_length: usize = self.cache_range.length();

        if self.cache_written && cache_length > 0 {
            self.file.seek(self.cache_range.start)?;
            let length: usize = self.cache_range.end - self.cache_range.start;
            let buffer: &[u8] = &self.cache_buffer[0..length];
            self.file.write(buffer)?;
            self.file.seek(start)?;
            self.cache_written = false;
        }

        Ok(())
    }

    fn cache_from_start(&mut self, start: usize) -> Result<(), DBError> {
        self.file.seek(start)?;
        self.position = start;

        let mut buffer: [u8; N] = [0; N];
        let length: usize = self.file.read(&mut buffer)?;
        self.position += length;

        if length == 0 {
            return Err(DBError::EndOfFileStream);
        }

        self.set_cache(buffer, start, self.position);
        Ok(())
    }

    fn cache_write_end(&mut self, buffer: &[u8], start: usize) -> Result<usize, DBError> {
        if self.cache_offset + buffer.len() > N {
            self.flush_cache_buffer()?;
            self.set_cache([0; N], start, start);
        }

        self.copy_into_cache(buffer);
        self.cache_range.set_end(start + buffer.len());
        return Ok(buffer.len());
    }

    fn write_to_file(&mut self, buffer: &[u8]) -> Result<usize, DBError> {
        let length: usize = self.file.write(buffer)?;
        self.position += length;
        self.cache_from_start(self.position - N)?;
        self.cache_offset = N;
        Ok(length)
    }
}

impl<'a, const N: usize> DBStreamCache<'a, N> {
    pub fn new(file: &'a mut FileBox) -> Self {
        file.seek(0).unwrap();

        let position: usize = 0;
        let cache_offset: usize = 0;
        let cache_range: Range<N> = Range::new();
        let cache_buffer: [u8; N] = [0; N];
        let cache_written: bool = false;

        Self {
            file,
            position,
            cache_offset,
            cache_range,
            cache_buffer,
            cache_written,
        }
    }

    pub fn read(&mut self, buffer: &mut [u8; BLOCK_SIZE]) -> Result<(), DBError> {
        let cache_length: usize = self.cache_range.length();
        if self.cache_offset < cache_length && cache_length > 0 {
            let head: usize = self.cache_offset;
            let tail: usize = head + BLOCK_SIZE;

            buffer.copy_from_slice(&self.cache_buffer[head..tail]);
            self.increment_cache_offset();
            return Ok(());
        }

        self.flush_cache_buffer()?;
        self.cache_from_start(self.position)?;
        self.read(buffer)?;
        Ok(())
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, DBError> {
        if buffer.len() > N {
            return self.write_to_file(buffer);
        } else if (self.cache_position() + buffer.len()) <= self.cache_range.end
            && self.cache_range.length() > 0
        {
            self.copy_into_cache(buffer);
            return Ok(buffer.len());
        }

        self.flush_cache_buffer()?;
        let start: usize = self.cache_range.start + self.cache_offset;
        let result: Result<(), DBError> = self.cache_from_start(start);
        if let Err(error) = result {
            match error {
                DBError::EndOfFileStream => {
                    return self.cache_write_end(buffer, start);
                }
                error => return Err(error),
            }
        }
        self.write(buffer)?;
        Ok(buffer.len())
    }

    pub fn seek_from_start(&mut self, start: usize) -> Result<usize, DBError> {
        let cache_st: usize = self.cache_range.start;
        let cache_en: usize = self.cache_range.end;

        if cache_st <= start && cache_en >= start {
            let offset: usize = start - cache_st;
            self.seek_cache_offset(offset);
            return Ok(start);
        }

        self.flush_cache_buffer()?;
        self.cache_from_start(start)?;
        Ok(start)
    }

    pub fn set_len(&mut self, size: usize) -> Result<(), DBError> {
        self.file.set_len(size)
    }
}

pub struct DBFileStream<'a, const N: usize> {
    stream: DBStreamCache<'a, N>,
    uid_serializer: UIDSerializer,
}

impl<'a, const N: usize> DBFileStream<'a, N> {
    fn rebuild_database(
        &mut self,
        st_pos1: &mut usize,
        en_pos1: &mut usize,
        mut uid: u32,
    ) -> Result<(), DBError> {
        while let Ok((_, en_pos2)) = self.get_chunk_bounds() {
            // Create UID block
            let uid_block: [u8; BLOCK_SIZE] = self.uid_serializer.serialize_uid(uid);

            // Get next chunk
            let mut next_chunk: Vec<u8> = self.iter_chunk()?;

            // Overwrite next chunk UID
            next_chunk[..BLOCK_SIZE].copy_from_slice(&uid_block);

            // Seek to current chunk start
            self.stream.seek_from_start(*st_pos1)?;

            // Overwrite with next chunk data
            self.stream.write(&next_chunk)?;

            // Seek to the end of next chunk
            self.stream.seek_from_start(en_pos2)?;

            // Set position of next chunk
            *st_pos1 = *st_pos1 + next_chunk.len();
            *en_pos1 = en_pos2;

            uid += 1;
        }
        Ok(())
    }
}

impl<'a, const N: usize> DBFileStream<'a, N> {
    pub fn new(file: &'a mut FileBox) -> Self {
        let stream: DBStreamCache<'_, N> = DBStreamCache::new(file);
        let uid_serializer: UIDSerializer = UIDSerializer::new();
        DBFileStream {
            stream,
            uid_serializer,
        }
    }

    pub fn get_chunk_bounds(&mut self) -> Result<(usize, usize), DBError> {
        let cache_st: usize = self.stream.cache_range.start + self.stream.cache_offset;

        for block in self.into_iter() {
            if let Ok(block) = block {
                if block == EOE_BLOCK {
                    break;
                }
                continue;
            }
            return Err(DBError::InvalidData);
        }

        let cache_en: usize = self.stream.cache_range.start + self.stream.cache_offset;
        self.stream.seek_from_start(cache_st)?;
        Ok((cache_st, cache_en))
    }

    pub fn append_end(&mut self, data: &[u8]) {
        while let Ok(_) = self.iter_chunk() {}
        let _ = self.stream.write(data);
    }

    pub fn last_chunk(&mut self) -> Option<Vec<u8>> {
        let mut last_chunk: Option<Vec<u8>> = None;
        while let Ok(chunk) = self.iter_chunk() {
            last_chunk = Some(chunk);
        }
        last_chunk
    }

    pub fn iter_chunk(&mut self) -> Result<Vec<u8>, DBError> {
        let mut data: Vec<u8> = Vec::new();

        for block in self.into_iter() {
            if let Ok(block) = block {
                data.extend(block);
                if block == EOE_BLOCK {
                    return Ok(data);
                }
                continue;
            }
        }
        Err(DBError::InvalidData)
    }

    pub fn remove_chunk(&mut self) -> Result<(), DBError> {
        let (mut st_pos1, mut en_pos1) = self.get_chunk_bounds()?;

        let current_chunk: Vec<u8> = self.iter_chunk()?;
        let current_uid_block: &[u8] = &current_chunk[..BLOCK_SIZE];
        let current_uid: u32 = self.uid_serializer.deserialize_uid(current_uid_block)?;

        let _ = self.rebuild_database(&mut st_pos1, &mut en_pos1, current_uid);
        self.stream.set_len(st_pos1)?;
        Ok(())
    }
}

impl<'a, const N: usize> Iterator for DBFileStream<'a, N> {
    type Item = Result<[u8; BLOCK_SIZE], DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
        let result: Result<(), DBError> = self.stream.read(&mut buffer);

        if let Ok(_) = result {
            return Some(Ok(buffer));
        }

        None
    }
}

impl<'a, const N: usize> Drop for DBFileStream<'a, N> {
    fn drop(&mut self) {
        self.stream.flush_cache_buffer().unwrap();
    }
}
