use std::fmt::Debug;

use crate::Error;

#[derive(Clone, Copy)]
pub struct StartSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> StartSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }
}

impl<'bytes> Debug for StartSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StartSection").finish()
    }
}
