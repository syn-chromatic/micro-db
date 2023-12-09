use crate::error::DBError;
use crate::structures::DBEntry;
use crate::BLOCK_SIZE;
use crate::EOE_BLOCK;

extern crate alloc;
use alloc::collections::BTreeSet;

use core::fmt::Debug;
use core::hash;
use core::marker::PhantomData;

use bincode::deserialize;
use bincode::serialize;
use bincode::ErrorKind;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct DBSerializer<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub _marker: PhantomData<&'a T>,
}

impl<'a, T> DBSerializer<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    fn bincode_serialize(&self, item: &T::Item) -> Result<Vec<u8>, DBError> {
        let bytes: Result<Vec<u8>, Box<ErrorKind>> = serialize(item);
        if let Ok(bytes) = bytes {
            return Ok(bytes);
        }
        Err(DBError::SerializeError)
    }

    fn bincode_deserialize(&self, bytes: &[u8]) -> Result<T::Item, DBError> {
        let item: Result<T::Item, Box<ErrorKind>> = deserialize(bytes);
        if let Ok(item) = item {
            return Ok(item);
        }
        Err(DBError::DeserializeError)
    }

    fn pad_serialized_chunk(&self, chunk: &[u8], buffer: &mut Vec<u8>) {
        let mut padded_chunk: Vec<u8> = chunk.to_vec();
        padded_chunk.resize(BLOCK_SIZE, 0);
        buffer.extend(padded_chunk);
    }
}

impl<'a, T> DBSerializer<'a, T>
where
    T: IntoIterator + Eq,
    T::Item: Serialize + DeserializeOwned + hash::Hash + Eq + Debug,
{
    pub fn new() -> Self {
        let _marker: PhantomData<&T> = PhantomData;
        Self { _marker }
    }

    pub fn serialize(&self, uid: u32, item: &T::Item) -> Result<Vec<u8>, DBError> {
        let mut buffer: Vec<u8> = Vec::new();
        let uid_block: Vec<u8> = self.serialize_uid(uid)?;
        buffer.extend(uid_block);

        let bytes: Vec<u8> = self.bincode_serialize(item)?;
        for block in bytes.chunks(BLOCK_SIZE) {
            if block.len() == BLOCK_SIZE {
                buffer.extend(block);
                continue;
            }
            self.pad_serialized_chunk(block, &mut buffer);
        }

        buffer.extend(EOE_BLOCK);
        Ok(buffer)
    }

    pub fn serialize_items(
        &self,
        mut uid: u32,
        items: BTreeSet<T::Item>,
    ) -> Result<Vec<u8>, DBError> {
        let mut buffer: Vec<u8> = Vec::new();

        for item in items.into_iter() {
            let bytes: Vec<u8> = self.bincode_serialize(&item)?;
            let uid_block: Vec<u8> = self.serialize_uid(uid)?;
            buffer.extend(uid_block);

            for block in bytes.chunks(BLOCK_SIZE) {
                if block.len() == BLOCK_SIZE {
                    buffer.extend(block);
                    continue;
                }
                self.pad_serialized_chunk(block, &mut buffer);
            }

            buffer.extend(EOE_BLOCK);
            uid += 1;
        }
        Ok(buffer)
    }

    pub fn deserialize(&self, buffer: &[u8]) -> Result<DBEntry<T::Item>, DBError> {
        let uid_block: &[u8] = &buffer[..BLOCK_SIZE];
        let uid: u32 = self.deserialize_uid(uid_block)?;

        let buffer: &[u8] = &buffer[BLOCK_SIZE..buffer.len() - BLOCK_SIZE];
        let item: T::Item = self.bincode_deserialize(buffer)?;

        let entry: DBEntry<T::Item> = DBEntry::new(uid, item);
        return Ok(entry);
    }

    pub fn deserialize_items(&self, buffer: &[u8]) -> Result<Vec<DBEntry<T::Item>>, DBError> {
        let mut items: Vec<DBEntry<T::Item>> = Vec::new();

        let mut uid: Option<u32> = None;
        let mut bytes: Vec<u8> = Vec::new();
        for (idx, block) in buffer.chunks(BLOCK_SIZE).enumerate() {
            if idx == 0 {
                uid = Some(self.deserialize_uid(block)?);
            }

            if block == EOE_BLOCK {
                let item: Result<T::Item, DBError> = self.bincode_deserialize(&bytes);
                if let Ok(item) = item {
                    if let Some(uid) = uid {
                        let entry: DBEntry<T::Item> = DBEntry::new(uid, item);
                        items.push(entry);
                    }
                }
                uid = None;
                bytes.clear();
                continue;
            }
            bytes.extend(block);
        }
        Ok(items)
    }

    pub fn serialize_uid(&self, uid: u32) -> Result<Vec<u8>, DBError> {
        let bytes: Result<Vec<u8>, Box<ErrorKind>> = serialize(&uid);
        if let Ok(mut bytes) = bytes {
            bytes.resize(BLOCK_SIZE, 0);
            return Ok(bytes);
        }
        Err(DBError::UIDSerializeError)
    }

    pub fn deserialize_uid(&self, buffer: &[u8]) -> Result<u32, DBError> {
        let uid: Result<u32, Box<ErrorKind>> = deserialize(buffer);
        if let Ok(uid) = uid {
            return Ok(uid);
        }

        Err(DBError::UIDDeserializeError)
    }
}
