use std::fmt::Debug;

use crate::{synth::sections::SynthDataCountSection, Bytes, Error};

#[derive(Clone, Copy)]
pub struct DataCountSection {
    data_count: u32,
}

impl DataCountSection {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let (data_count, bytes) = bytes.advance_u32()?;
        Ok(Self { data_count })
    }

    pub(crate) fn into_synth(self) -> SynthDataCountSection {
        SynthDataCountSection {
            data_count: self.data_count,
        }
    }

    pub fn data_count(&self) -> u32 {
        self.data_count
    }
}

impl Debug for DataCountSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataCountSection").finish()
    }
}
