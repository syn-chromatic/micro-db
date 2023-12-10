use crate::error;
use crate::serializer;
use crate::stream;
use crate::CACHE_SIZE;

use error::DBError;
use serializer::DBSerializer;
use stream::DBFileStream;

use core::fmt::Debug;
use core::hash;

use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub struct DBEntry<T>
where
    T: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub uid: u32,
    pub item: T,
}

impl<T> DBEntry<T>
where
    T: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub fn new(uid: u32, item: T) -> Self {
        Self { uid, item }
    }
}

pub struct DBIterator<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    stream: DBFileStream<CACHE_SIZE>,
    serializer: DBSerializer<'a, T>,
}

impl<'a, T> DBIterator<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub fn new(stream: DBFileStream<CACHE_SIZE>, serializer: DBSerializer<'a, T>) -> Self {
        Self { stream, serializer }
    }
}

impl<'a, T> Iterator for DBIterator<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    type Item = Result<DBEntry<T::Item>, DBError>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.stream.next_chunk();
        if let Ok(chunk) = chunk {
            let entry = self.serializer.deserialize(&chunk);
            return Some(entry);
        }
        None
    }
}

