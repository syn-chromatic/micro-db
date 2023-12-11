use crate::error::DBError;
use crate::traits::CPathBox;
use crate::traits::CPathTrait;
use crate::traits::FileTrait;
use crate::traits::OpenFileBox;
use crate::traits::OpenFileTrait;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

impl FileTrait for File {
    fn create(path: &dyn CPathTrait) -> Result<Box<dyn FileTrait>, DBError>
    where
        Self: Sized,
    {
        let result: Result<File, Error> = File::create(path.as_str());
        if let Ok(file) = result {
            return Ok(Box::new(file));
        }
        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), DBError> {
        let result: Result<(), Error> = <Self as Read>::read_exact(self, buffer);
        if result.is_ok() {
            return Ok(());
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DBError> {
        let result: Result<usize, Error> = <Self as Write>::write(self, buffer);
        if let Ok(result) = result {
            return Ok(result);
        }

        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }

    fn write_all(&mut self, buffer: &[u8]) -> Result<(), DBError> {
        let result: Result<(), Error> = <Self as Write>::write_all(self, buffer);
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
        let result: Result<(), Error> = self.set_len(size as u64);
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

#[derive(Clone)]
pub struct OpenFile {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
}

impl OpenFileTrait for OpenFile {
    fn new() -> OpenFileBox
    where
        Self: Sized,
    {
        let read: bool = false;
        let write: bool = false;
        let append: bool = false;
        let truncate: bool = false;
        let create: bool = false;
        Box::new(Self {
            read,
            write,
            append,
            truncate,
            create,
        })
    }

    fn boxed(&self) -> OpenFileBox {
        Box::new(self.clone())
    }

    fn read(&mut self, read: bool) {
        self.read = read;
    }

    fn write(&mut self, write: bool) {
        self.write = write;
    }

    fn append(&mut self, append: bool) {
        self.append = append;
    }

    fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }

    fn create(&mut self, create: bool) {
        self.create = create;
    }

    fn reset(&mut self) {
        self.read = false;
        self.write = false;
        self.append = false;
        self.truncate = false;
        self.create = false;
    }

    fn open(&self, path: &dyn CPathTrait) -> Result<crate::traits::FileBox, DBError> {
        let mut open_options: OpenOptions = OpenOptions::new();
        open_options.read(self.read);
        open_options.write(self.write);
        open_options.append(self.append);
        open_options.truncate(self.truncate);
        open_options.create(self.create);

        let result: Result<File, Error> = open_options.open(path.as_str());
        if let Ok(file) = result {
            return Ok(Box::new(file));
        }
        let db_error: DBError = result.unwrap_err().into();
        Err(db_error)
    }
}

#[derive(Clone)]
pub struct CPath {
    path: String,
}

impl CPath {
    pub fn new(path: &str) -> CPath {
        let path: String = path.to_string();
        CPath { path }
    }
}

impl CPathTrait for CPath {
    fn as_str(&self) -> &str {
        &self.path
    }

    fn boxed(&self) -> CPathBox {
        Box::new(self.clone())
    }
}
