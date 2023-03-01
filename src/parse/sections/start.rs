use std::fmt::Debug;

use crate::{synth::sections::SynthStartSection, Bytes, Error};

#[derive(Clone, Copy)]
pub struct StartSection {
    pub(crate) start: u32,
}

impl StartSection {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            start: bytes.advance_u32()?.0,
        })
    }

    pub(crate) fn into_synth(self) -> SynthStartSection {
        SynthStartSection { start: self.start }
    }

    pub fn start(&self) -> u32 {
        self.start
    }
}

impl Debug for StartSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StartSection")
            .field("start", &self.start)
            .finish()
    }
}
