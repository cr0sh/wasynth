use std::fmt::Debug;

use crate::{Bytes, Error};

#[derive(Clone, Copy)]
pub struct FunctionSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> FunctionSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub fn type_indices(&self) -> Result<impl Iterator<Item = Result<u32, Error>> + '_, Error> {
        self.bytes.advance_vector(<&[u8]>::advance_u32)
    }
}

impl<'bytes> Debug for FunctionSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionSection").finish()
    }
}
