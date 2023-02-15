use std::fmt::Debug;

use crate::{synth::sections::SynthTableSection, wasm_types::TableType, Bytes, Error};

#[derive(Clone, Copy)]
pub struct TableSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> TableSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthTableSection, Error> {
        Ok(SynthTableSection {
            tables: self.tables()?.collect::<Result<Vec<_>, Error>>()?,
        })
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
