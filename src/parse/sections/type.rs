use std::fmt::Debug;

use crate::{wasm_types::FuncType, Bytes, Error};

#[derive(Clone, Copy)]
pub struct TypeSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> TypeSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub fn types(&self) -> Result<impl Iterator<Item = Result<FuncType, Error>> + '_, Error> {
        self.bytes.advance_vector(FuncType::from_bytes)
    }
}

impl<'bytes> Debug for TypeSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeSection").finish()
    }
}
