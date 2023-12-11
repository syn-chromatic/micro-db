#![cfg_attr(feature = "embedded", no_std)]
#![cfg_attr(feature = "embedded", no_main)]

#[cfg(any(
    all(feature = "no-std", not(feature = "alloc")),
    all(not(feature = "no-std"), feature = "alloc")
))]
compile_error!(
    "Unable to build with no-std or alloc exclusively,
    this crate requires both to be enabled if either are selected,
    use 'embedded' feature instead."
);

pub const BLOCK_SIZE: usize = 4;
pub const CACHE_SIZE: usize = 2048;
pub const EOE_BLOCK: [u8; BLOCK_SIZE] = [0xFF, 0xFE, 0xFD, 0xFC];

pub mod db;
pub mod error;
pub mod serializer;
pub mod stream;
pub mod structures;
pub mod traits;

#[cfg(feature = "std")]
pub mod impls;
#[cfg(feature = "std")]
pub mod tests;
#[cfg(feature = "std")]
pub mod utils;
