use std::fmt::Debug;

use crate::{Bytes, Error};

#[derive(Clone, Copy)]
pub struct CustomSection<'bytes> {
    name: &'bytes str,
    bytes: &'bytes [u8],
}

impl<'bytes> CustomSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        let (name, bytes) = bytes.advance_name()?;
        Ok(Self { name, bytes })
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

impl<'bytes> Debug for CustomSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomSection")
            .field("name", &self.name)
            .field("bytes", &"<snip>")
            .finish()
    }
}
