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

use core::hash::Hash;
use core::marker::PhantomData;

use error::DBError;
use serializer::DBSerializer;
use serializer::UIDSerializer;
use stream::DBFileStream;
use structures::DBChunkIterator;
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
    T::Item: Encode + Decode + Hash + Eq,
{
    pub path: CPathBox,
    pub open: OpenFileBox,
    pub file: Option<(FileBox, [bool; 3])>,
    pub marker: PhantomData<&'a T>,
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + Hash + Eq,
{
    fn get_uid_from_chunk(chunk: Option<Vec<u8>>) -> u32 {
        if let Some(chunk) = chunk {
            let first_block: &[u8] = &chunk[..BLOCK_SIZE];
            let uid: u32 = UIDSerializer::new().deserialize_uid(first_block).unwrap();
            return uid + 1;
        }
        0
    }
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + Hash + Eq,
{
    pub fn new(path: &dyn CPathTrait, open: OpenFileBox) -> Self {
        let path: CPathBox = path.boxed();
        let file: Option<(FileBox, [bool; 3])> = None;
        let marker: PhantomData<&T> = PhantomData;
        Database {
            path,
            open,
            file,
            marker,
        }
    }

    pub fn query<Q: PartialEq, V: Fn(&T::Item) -> &Q>(
        &mut self,
        value: V,
        query: Q,
    ) -> Result<DBEntry<T::Item>, DBError> {
        let iterator: DBIterator<'_, BTreeSet<<T as IntoIterator>::Item>> = self.get_iterator()?;

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if value(&entry.item) == &query {
                    return Ok(entry);
                }
            }
        }

        Err(DBError::EntryNotFound)
    }

    pub fn contains(&mut self, item: &T::Item) -> Result<bool, DBError> {
        let iterator: DBIterator<'_, BTreeSet<<T as IntoIterator>::Item>> = self.get_iterator()?;

        for entry in iterator.into_iter() {
            if let Ok(entry) = entry {
                if &entry.item == item {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    pub fn get_by_uid(&mut self, uid: u32) -> Result<DBEntry<T::Item>, DBError> {
        let iterator: DBIterator<'_, BTreeSet<<T as IntoIterator>::Item>> = self.get_iterator()?;

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
        self.open([true, true, false]);
        if let Some((file, _)) = &mut self.file {
            let mut db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
            for _ in 0..uid {
                db_stream.iter_chunk()?;
            }

            db_stream.remove_chunk()?;
        }
        Ok(())
    }

    pub fn add_entry(&mut self, item: &T::Item) -> Result<(), DBError> {
        self.open([true, true, true]);
        if let Some((file, _)) = &mut self.file {
            let mut db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
            let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

            let last_chunk: Option<Vec<u8>> = db_stream.last_chunk();
            let uid: u32 = Self::get_uid_from_chunk(last_chunk);

            let data: Vec<u8> = db_serializer.serialize(uid, item).unwrap();
            db_stream.append_end(&data);
        }
        Ok(())
    }

    pub fn add_entries(&mut self, items: BTreeSet<T::Item>) -> Result<(), DBError> {
        self.open([true, true, true]);
        if let Some((file, _)) = &mut self.file {
            let mut db_stream: DBFileStream<CACHE_SIZE> = DBFileStream::new(file);
            let db_serializer: DBSerializer<'_, T> = DBSerializer::new();

            let last_chunk: Option<Vec<u8>> = db_stream.last_chunk();
            let uid: u32 = Self::get_uid_from_chunk(last_chunk);

            let data: Vec<u8> = db_serializer.serialize_items(uid, items).unwrap();
            db_stream.append_end(&data);
        }
        Ok(())
    }

    pub fn get_iterator(&mut self) -> Result<DBIterator<'_, BTreeSet<T::Item>>, DBError> {
        self.open([true, false, false]);
        if let Some((file, _)) = &mut self.file {
            let iterator: DBIterator<'_, BTreeSet<<T as IntoIterator>::Item>> =
                DBIterator::<BTreeSet<T::Item>>::from_file(file);
            return Ok(iterator);
        }
        Err(DBError::FailedToRetrieveIterator)
    }

    pub fn get_chunk_iterator(&mut self) -> Result<DBChunkIterator<'_>, DBError> {
        self.open([true, false, false]);
        if let Some((file, _)) = &mut self.file {
            let iterator: DBChunkIterator<'_> = DBChunkIterator::from_file(file);
            return Ok(iterator);
        }
        Err(DBError::FailedToRetrieveIterator)
    }
}

impl<'a, T> Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + Hash + Eq,
{
    fn open(&mut self, rwc: [bool; 3]) {
        if let Some((_, file_rwc)) = self.file.as_ref() {
            if file_rwc == &rwc {
                return;
            }
            self.close();
        }
        let file: FileBox = self.get_file_from_rwc(&rwc);
        self.file = Some((file, rwc));
    }

    fn close(&mut self) {
        if let Some((file, _)) = self.file.take() {
            let _ = file.close();
            self.file = None;
        }
    }

    fn get_file_from_rwc(&mut self, rwc: &[bool; 3]) -> FileBox {
        if rwc[0] {
            self.open.read(true);
        }
        if rwc[1] {
            self.open.write(true);
        }
        if rwc[2] {
            self.open.create(true);
        }
        let file: FileBox = self.open.open(&*self.path).unwrap();
        file
    }
}

impl<'a, T> Drop for Database<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Encode + Decode + Hash + Eq,
{
    fn drop(&mut self) {
        self.close();
    }
}
