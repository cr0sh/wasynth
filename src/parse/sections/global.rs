use std::fmt::Debug;

use crate::{synth::sections::SynthGlobalSection, Error};

#[derive(Clone, Copy)]
pub struct GlobalSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> GlobalSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> SynthGlobalSection {
        SynthGlobalSection {
            bytes: self.bytes.to_owned(),
        }
    }
}

impl<'bytes> Debug for GlobalSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalSection").finish()
    }
}
