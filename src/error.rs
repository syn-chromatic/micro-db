#[derive(Debug)]
pub enum DBError {
    SerializeError,
    DeserializeError,
    UIDSerializeError,
    UIDDeserializeError,
    InvalidData,
    EntryNotFound,
    IOError(String),
}
