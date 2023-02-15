use std::fmt::Debug;

use crate::{synth::sections::SynthExportSection, Error};

#[derive(Clone, Copy)]
pub struct ExportSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> ExportSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> SynthExportSection {
        SynthExportSection {
            bytes: self.bytes.to_owned(),
        }
    }
}

impl<'bytes> Debug for ExportSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportSection").finish()
    }
}
