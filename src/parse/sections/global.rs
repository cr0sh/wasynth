use std::fmt::Debug;

use crate::{wasm_types::GlobalType, Error};

#[derive(Clone, Copy)]
pub struct GlobalSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> GlobalSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }
}

impl<'bytes> Debug for GlobalSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalSection").finish()
    }
}
