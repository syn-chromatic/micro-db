use crate::error;
use crate::serializer;
use crate::stream;
use crate::structures;
use crate::BLOCK_SIZE;

extern crate alloc;
use alloc::collections::BTreeSet;

use core::fmt::Debug;
use core::hash;
use core::marker::PhantomData;

use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

use error::DBError;
use serializer::DBSerializer;
use stream::DBFileStream;
use structures::DBEntry;
use structures::DBIterator;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub path: PathBuf,
    pub strict_dupes: bool,
    pub _marker: PhantomData<&'a T>,
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    fn get_uid_from_chunk(
        &self,
        chunk: Option<Vec<u8>>,
        db_serializer: &DBSerializer<'_, T>,
    ) -> usize {
        if let Some(chunk) = chunk {
            let first_block: &[u8] = &chunk[..BLOCK_SIZE];
            let entry: usize = db_serializer.deserialize_uid(first_block).unwrap();
            return entry + 1;
        }
        0
    }
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub fn new(path: impl Into<PathBuf>, strict_dupes: bool) -> Self {
        Database {
            path: path.into(),
            strict_dupes,
            _marker: PhantomData,
        }
    }

    pub fn get_iterator(&self) -> DBIterator<'_, T> {
        let file: File = File::open(&self.path).unwrap();
        let db_stream: DBFileStream = DBFileStream::new(file);
        let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

        let iterator: DBIterator<'_, T> = DBIterator::new(db_stream, db_serializer);
        iterator
    }

    pub fn get_entry(&self, uid: usize) -> Result<DBEntry<T::Item>, DBError> {
        let iterator: DBIterator<'_, T> = self.get_iterator();

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if entry.uid == uid {
                    return Ok(entry);
                }
            }
        }

        Err(DBError::EntryNotFound)
    }

    pub fn contains(&self, query: &T::Item) -> bool {
        let iterator: DBIterator<'_, T> = self.get_iterator();

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if &entry.item == query {
                    return true;
                }
            }
        }
        false
    }

    pub fn add_item(&self, item: &T::Item) {
        let file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)
            .unwrap();

        let mut db_stream: DBFileStream = DBFileStream::new(file);
        let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

        let last_chunk: Option<Vec<u8>> = db_stream.last_chunk();
        let uid: usize = self.get_uid_from_chunk(last_chunk, &db_serializer);

        let data: Vec<u8> = db_serializer.serialize(uid, item).unwrap();
        db_stream.append_end(&data);
    }

    pub fn add_items(&self, items: BTreeSet<T::Item>) {
        let file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)
            .unwrap();

        let mut db_stream: DBFileStream = DBFileStream::new(file);
        let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

        let last_chunk: Option<Vec<u8>> = db_stream.last_chunk();
        let uid: usize = self.get_uid_from_chunk(last_chunk, &db_serializer);

        let data: Vec<u8> = db_serializer.serialize_items(uid, items).unwrap();
        db_stream.append_end(&data);
    }
}