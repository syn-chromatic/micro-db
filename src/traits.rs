use crate::error::DBError;
use core::ops::Deref;

pub trait FileTrait {
    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), DBError>;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DBError>;
    fn write_all(&mut self, buffer: &[u8]) -> Result<(), DBError>;
    fn seek(&mut self, position: usize) -> Result<usize, DBError>;
    fn stream_position(&mut self) -> Result<usize, DBError>;
    fn set_len(&self, size: usize) -> Result<(), DBError>;
}

pub type PathBufBox = Box<dyn PathBufTrait>;

pub trait PathBufTrait {
    fn as_str(&self) -> &str;
    fn clone_box(&self) -> Box<dyn PathBufTrait>;
}

impl Deref for dyn PathBufTrait {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.as_str()
    }
}
