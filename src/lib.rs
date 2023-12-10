pub const BLOCK_SIZE: usize = 4;
pub const CACHE_SIZE: usize = 2048;
pub const EOE_BLOCK: [u8; BLOCK_SIZE] = [0xFF, 0xFE, 0xFD, 0xFC];

pub mod db;
pub mod error;
pub mod impls;
pub mod serializer;
pub mod stream;
pub mod structures;
pub mod tests;
pub mod traits;
pub mod utils;
