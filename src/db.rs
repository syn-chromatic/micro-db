use crate::error;
use crate::serializer;
use crate::stream;
use crate::structures;
use crate::traits;
use crate::BLOCK_SIZE;
use crate::CACHE_SIZE;

extern crate alloc;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use core::fmt::Debug;
use core::hash;
use core::marker::PhantomData;

use error::DBError;
use serializer::DBSerializer;
use stream::DBFileStream;
use structures::DBEntry;
use structures::DBIterator;
use traits::CPathBox;
use traits::CPathTrait;
use traits::FileBox;

use traits::OpenFileBox;

use bincode::Decode;
use bincode::Encode;

pub struct Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + hash::Hash + Eq + Debug,
{
    pub path: CPathBox,
    pub open: OpenFileBox,
    pub strict_dupes: bool,
    pub _marker: PhantomData<&'a T>,
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + hash::Hash + Eq + Debug,
{
    fn get_uid_from_chunk(
        &self,
        chunk: Option<Vec<u8>>,
        db_serializer: &DBSerializer<'_, T>,
    ) -> u32 {
        if let Some(chunk) = chunk {
            let first_block: &[u8] = &chunk[..BLOCK_SIZE];
            let uid: u32 = db_serializer.deserialize_uid(first_block).unwrap();
            return uid + 1;
        }
        0
    }
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + hash::Hash + Eq + Debug,
{
    pub fn new(path: &dyn CPathTrait, open: OpenFileBox, strict_dupes: bool) -> Self {
        let path: CPathBox = path.boxed();
        Database {
            path,
            open,
            strict_dupes,
            _marker: PhantomData,
        }
    }

    pub fn query<Q: PartialEq, V: Fn(&T::Item) -> &Q>(
        &mut self,
        value: V,
        query: Q,
    ) -> Result<DBEntry<T::Item>, DBError> {
        let iterator: DBIterator<'_, T> = self.iterator();

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if value(&entry.item) == &query {
                    return Ok(entry);
                }
            }
        }

        Err(DBError::EntryNotFound)
    }

    pub fn contains(&mut self, item: &T::Item) -> bool {
        let iterator: DBIterator<'_, T> = self.iterator();

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if &entry.item == item {
                    return true;
                }
            }
        }
        false
    }

    pub fn iterator(&mut self) -> DBIterator<'_, T> {
        self.open.reset();
        self.open.read(true);
        let file: FileBox = self.open.open(&*self.path).unwrap();

        let db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
        let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

        let iterator: DBIterator<'_, T> = DBIterator::new(db_stream, db_serializer);
        iterator
    }

    pub fn get_by_uid(&mut self, uid: u32) -> Result<DBEntry<T::Item>, DBError> {
        let iterator: DBIterator<'_, T> = self.iterator();

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if entry.uid == uid {
                    return Ok(entry);
                }
            }
        }

        Err(DBError::EntryNotFound)
    }

    pub fn remove_by_uid(&mut self, uid: u32) -> Result<(), DBError> {
        self.open.reset();
        self.open.read(true);
        self.open.write(true);
        let file: FileBox = self.open.open(&*self.path).unwrap();

        let mut db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
        for _ in 0..uid {
            db_stream.next_chunk()?;
        }

        db_stream.remove_chunk()?;
        Ok(())
    }

    pub fn add_entry(&mut self, item: &T::Item) {
        self.open.reset();
        self.open.read(true);
        self.open.write(true);
        self.open.create(true);

        let file: FileBox = self.open.open(&*self.path).unwrap();

        let mut db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
        let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

        let last_chunk: Option<Vec<u8>> = db_stream.last_chunk();
        let uid: u32 = self.get_uid_from_chunk(last_chunk, &db_serializer);

        let data: Vec<u8> = db_serializer.serialize(uid, item).unwrap();
        db_stream.append_end(&data);
    }

    pub fn add_entries(&mut self, items: BTreeSet<T::Item>) {
        self.open.reset();
        self.open.read(true);
        self.open.write(true);
        self.open.create(true);

        let file: FileBox = self.open.open(&*self.path).unwrap();

        let mut db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
        let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

        let last_chunk: Option<Vec<u8>> = db_stream.last_chunk();
        let uid: u32 = self.get_uid_from_chunk(last_chunk, &db_serializer);

        let data: Vec<u8> = db_serializer.serialize_items(uid, items).unwrap();
        db_stream.append_end(&data);
    }
}
