use crate::structures::FileTrait;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

impl FileTrait for File {
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        <Self as Read>::read_exact(self, buffer)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, Error> {
        <Self as Write>::write(self, buffer)
    }

    fn write_all(&mut self, buffer: &[u8]) -> Result<(), Error> {
        <Self as Write>::write_all(self, buffer)
    }

    fn seek(&mut self, position: usize) -> Result<usize, Error> {
        let result: Result<u64, Error> =
            <Self as Seek>::seek(self, SeekFrom::Start(position as u64));
        if let Ok(result) = result {
            return Ok(result as usize);
        }
        Err(result.unwrap_err())
    }

    fn stream_position(&mut self) -> Result<usize, Error> {
        let result: Result<u64, Error> = <Self as Seek>::stream_position(self);
        if let Ok(result) = result {
            return Ok(result as usize);
        }
        Err(result.unwrap_err())
    }

    fn set_len(&self, size: usize) -> Result<(), Error> {
        self.set_len(size as u64)
    }
}