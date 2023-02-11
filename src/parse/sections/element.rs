use std::fmt::Debug;

use crate::Error;

#[derive(Clone, Copy)]
pub struct ElementSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> ElementSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }
}

impl<'bytes> Debug for ElementSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementSection").finish()
    }
}
