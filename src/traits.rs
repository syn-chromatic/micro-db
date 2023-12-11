extern crate alloc;
use alloc::boxed::Box;

use crate::error::DBError;
use core::ops::Deref;

pub type FileBox = Box<dyn FileTrait>;
pub type CPathBox = Box<dyn CPathTrait>;
pub type OpenFileBox = Box<dyn OpenFileTrait>;

pub trait FileTrait {
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), DBError>;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DBError>;
    fn write_all(&mut self, buffer: &[u8]) -> Result<(), DBError>;
    fn seek(&mut self, position: usize) -> Result<usize, DBError>;
    fn set_len(&self, size: usize) -> Result<(), DBError>;
}

pub trait OpenFileTrait {
    fn new() -> OpenFileBox
    where
        Self: Sized;
    fn boxed(&self) -> OpenFileBox;

    fn read(&mut self, read: bool);
    fn write(&mut self, write: bool);
    fn append(&mut self, append: bool);
    fn truncate(&mut self, truncate: bool);
    fn create(&mut self, create: bool);
    fn reset(&mut self);
    fn open(&self, path: &dyn CPathTrait) -> Result<FileBox, DBError>;
}

pub trait CPathTrait {
    fn as_str(&self) -> &str;
    fn boxed(&self) -> CPathBox;
}

impl Deref for dyn CPathTrait {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.as_str()
    }
}
