use crate::error::DBError;
use crate::structures::FileTrait;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

impl FileTrait for File {
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), DBError> {
        let result = <Self as Read>::read_exact(self, buffer);
        if result.is_ok() {
            return Ok(());
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DBError> {
        let result = <Self as Write>::write(self, buffer);
        if let Ok(result) = result {
            return Ok(result);
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn write_all(&mut self, buffer: &[u8]) -> Result<(), DBError> {
        let result = <Self as Write>::write_all(self, buffer);
        if result.is_ok() {
            return Ok(());
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn seek(&mut self, position: usize) -> Result<usize, DBError> {
        let result: Result<u64, Error> =
            <Self as Seek>::seek(self, SeekFrom::Start(position as u64));
        if let Ok(result) = result {
            return Ok(result as usize);
        }
        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn stream_position(&mut self) -> Result<usize, DBError> {
        let result: Result<u64, Error> = <Self as Seek>::stream_position(self);
        if let Ok(result) = result {
            return Ok(result as usize);
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn set_len(&self, size: usize) -> Result<(), DBError> {
        let result = self.set_len(size as u64);
        if result.is_ok() {
            return Ok(());
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }
}

impl Into<DBError> for std::io::Error {
    fn into(self) -> DBError {
        let string: String = self.to_string();
        DBError::IOError(string)
    }
}
