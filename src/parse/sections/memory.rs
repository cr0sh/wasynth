use std::fmt::Debug;

use crate::{wasm_types::MemType, Bytes, Error};

#[derive(Clone, Copy)]
pub struct MemorySection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> MemorySection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub fn memories(&self) -> Result<impl Iterator<Item = Result<MemType, Error>> + '_, Error> {
        self.bytes.advance_vector(MemType::from_bytes)
    }
}

impl<'bytes> Debug for MemorySection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemorySection").finish()
    }
}
