use crate::error;
use crate::serializer;
use crate::stream;
use crate::traits;
use crate::CACHE_SIZE;

use error::DBError;
use serializer::DBSerializer;
use stream::DBFileStream;
use traits::FileBox;

use core::fmt::Debug;
use core::hash;

use bincode::Decode;
use bincode::Encode;

#[derive(Debug)]
pub struct DBEntry<T>
where
    T: Encode + Decode + hash::Hash + Eq + Debug,
{
    pub uid: u32,
    pub item: T,
}

impl<T> DBEntry<T>
where
    T: Encode + Decode + hash::Hash + Eq + Debug,
{
    pub fn new(uid: u32, item: T) -> Self {
        Self { uid, item }
    }
}

pub struct DBIterator<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + hash::Hash + Eq + Debug,
{
    stream: DBFileStream<'a, CACHE_SIZE>,
    serializer: DBSerializer<'a, T>,
}

impl<'a, T> DBIterator<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + hash::Hash + Eq + Debug,
{
    pub fn new(stream: DBFileStream<'a, CACHE_SIZE>, serializer: DBSerializer<'a, T>) -> Self {
        Self { stream, serializer }
    }

    pub fn from_file(file: &'a mut FileBox) -> Self {
        let stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
        let serializer: DBSerializer<'_, T> = DBSerializer::new();
        Self { stream, serializer }
    }
}

impl<'a, T> Iterator for DBIterator<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + hash::Hash + Eq + Debug,
{
    type Item = Result<DBEntry<T::Item>, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.stream.iter_chunk();
        if let Ok(chunk) = chunk {
            let entry = self.serializer.deserialize(&chunk);
            return Some(entry);
        }
        None
    }
}

pub struct DBChunkIterator<'a> {
    stream: DBFileStream<'a, CACHE_SIZE>,
}

impl<'a> DBChunkIterator<'a> {
    pub fn from_file(file: &'a mut FileBox) -> Self {
        let stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
        Self { stream }
    }
}

impl<'a> Iterator for DBChunkIterator<'a> {
    type Item = Result<Vec<u8>, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.stream.iter_chunk();
        if let Ok(chunk) = chunk {
            return Some(Ok(chunk));
        }
        None
    }
}
