#[derive(Debug)]
pub enum DBError {
    SerializeError,
    DeserializeError,
    UIDSerializeError,
    UIDDeserializeError,
    InvalidData,
    EntryNotFound,
    IOError(std::io::Error),
}

impl From<std::io::Error> for DBError {
    fn from(e: std::io::Error) -> Self {
        DBError::IOError(e)
    }
}
