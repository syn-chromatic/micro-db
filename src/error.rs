extern crate alloc;
use alloc::string::String;

#[derive(Debug)]
pub enum DBError {
    SerializeError,
    DeserializeError,
    UIDSerializeError,
    UIDDeserializeError,
    InvalidData,
    EntryNotFound,
    FailedToRetrieveIterator,
    IOError(String),
    EndOfFileStream,
}
