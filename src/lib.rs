pub const BLOCK_SIZE: usize = 4;
pub const CACHE_SIZE: usize = 1024;
pub const EOE_BLOCK: [u8; BLOCK_SIZE] = [0xFF, 0xFE, 0xFD, 0xFC];

pub mod db;
pub mod error;
pub mod file;
pub mod serializer;
pub mod stream;
pub mod structures;
pub mod utils;
