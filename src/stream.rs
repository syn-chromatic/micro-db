use crate::structures::FileTrait;
use crate::BLOCK_SIZE;
use crate::EOE_BLOCK;

use std::io;

pub struct Position<const N: usize> {
    start: usize,
    end: usize,
}

impl<const N: usize> Position<N> {
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
    position: Position<N>,
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
        let position: Position<N> = Position::new();
        let cache: [u8; N] = [0; N];
        Self {
            ready,
            offset,
            position,
            cache,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn seek_from_offset(&mut self, offset: usize) {
        self.offset = offset;
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

    pub fn set_cache(&mut self, cache: [u8; N], start: usize, end: usize) {
        self.cache = cache;
        self.ready = true;
        self.offset = 0;

        self.position.set_start(start);
        self.position.set_end(end);
    }

    pub fn clear(&mut self) {
        self.ready = false;
    }
}

pub struct DBFileStream<const N: usize> {
    file: Box<dyn FileTrait>,
    cache: DBStreamCache<N>,
}

impl<const N: usize> DBFileStream<N> {
    fn update_cache(&mut self) -> Result<(), io::Error> {
        if !self.cache.is_ready() {
            let st_position = self.file.stream_position()?;
            for _ in self.into_iter() {
                break;
            }
            self.seek_from_start(st_position)?;
        }
        Ok(())
    }

    fn seek_from_start(&mut self, start: usize) -> Result<(), io::Error> {
        let cache_st: usize = self.cache.position.start;
        let cache_en: usize = self.cache.position.end;

        if cache_st <= start && cache_en > start {
            let offset: usize = start - cache_st;
            self.cache.seek_from_offset(offset);
        }

        self.file.seek(start)?;
        Ok(())
    }

    fn write(&mut self, buffer: &[u8]) -> Result<(), io::Error> {
        self.file.write(buffer)?;
        Ok(())
    }

    fn rebuild_database(
        &mut self,
        st_pos1: &mut usize,
        en_pos1: &mut usize,
    ) -> Result<(), io::Error> {
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

impl<const N: usize> DBFileStream<N> {
    pub fn new(file: Box<dyn FileTrait>) -> Self {
        let cache: DBStreamCache<N> = DBStreamCache::new();
        DBFileStream { file, cache }
    }

    pub fn get_chunk_bounds(&mut self) -> Result<(usize, usize), io::Error> {
        self.update_cache()?;
        let cache_st: usize = self.cache.offset + self.cache.position.start;
        for block in self.into_iter() {
            if let Ok(block) = block {
                if block == EOE_BLOCK {
                    break;
                }
                continue;
            }
            return Err(io::ErrorKind::InvalidData.into());
        }

        let cache_en: usize = self.cache.offset + self.cache.position.start;
        self.seek_from_start(cache_st)?;
        Ok((cache_st, cache_en))
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

    pub fn next_chunk(&mut self) -> Result<Vec<u8>, io::Error> {
        let mut data: Vec<u8> = Vec::new();

        for block in self.into_iter() {
            if let Ok(block) = block {
                data.extend(block);
                if block == EOE_BLOCK {
                    return Ok(data);
                }
                continue;
            }
            return Err(io::ErrorKind::Unsupported.into());
        }
        Err(io::ErrorKind::AlreadyExists.into())
    }

    pub fn remove_chunk(&mut self) -> Result<(), io::Error> {
        let (mut st_pos1, mut en_pos1) = self.get_chunk_bounds()?;
        self.rebuild_database(&mut st_pos1, &mut en_pos1)?;
        self.file.set_len(st_pos1)?;
        Ok(())
    }
}

impl<const N: usize> Iterator for DBFileStream<N> {
    type Item = Result<[u8; BLOCK_SIZE], io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_ready() {
            let bytes: [u8; BLOCK_SIZE] = self.cache.get_chunk();
            return Some(Ok(bytes));
        }

        if let Ok(start) = self.file.stream_position() {
            let start: usize = start as usize;
            let end: usize = start + N;

            let mut buffer: [u8; N] = [0; N];
            let result: Result<(), io::Error> = self.file.read_exact(&mut buffer);
            if let Err(error) = result {
                if buffer.iter().all(|&x| x == 0) {
                    return Some(Err(error));
                }
            }

            self.cache.set_cache(buffer, start, end);
            let bytes: [u8; BLOCK_SIZE] = self.cache.get_chunk();
            return Some(Ok(bytes));
        }
        None
    }
}
