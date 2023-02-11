use std::fmt::Debug;

use crate::{wasm_types::TableType, Bytes, Error};

#[derive(Clone, Copy)]
pub struct TableSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> TableSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub fn tables(&self) -> Result<impl Iterator<Item = Result<TableType, Error>> + '_, Error> {
        self.bytes.advance_vector(TableType::from_bytes)
    }
}

impl<'bytes> Debug for TableSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableSection").finish()
    }
}
