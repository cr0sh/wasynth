use std::fmt::Debug;

use crate::{
    wasm_types::{GlobalType, MemType, TableType},
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct ImportSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> ImportSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub fn imports(&self) -> Result<impl Iterator<Item = Result<Import, Error>> + '_, Error> {
        self.bytes.advance_vector(Import::from_bytes)
    }
}

impl<'bytes> Debug for ImportSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportSection").finish()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Import<'bytes> {
    module: &'bytes str,
    name: &'bytes str,
    description: ImportDescription,
}

impl<'bytes> Import<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &[u8]), Error> {
        let (module, bytes) = bytes.advance_name()?;
        let (name, bytes) = bytes.advance_name()?;
        let (description, bytes) = ImportDescription::from_bytes(bytes)?;
        Ok((
            Self {
                module,
                name,
                description,
            },
            bytes,
        ))
    }

    pub fn module(&self) -> &str {
        self.module
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn description(&self) -> ImportDescription {
        self.description
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportDescription {
    Type(u32),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}

impl ImportDescription {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[id], bytes) = bytes.advance()?;
        match id {
            0x00 => {
                let (ty, bytes) = bytes.advance_u32()?;
                Ok((Self::Type(ty), bytes))
            }
            0x01 => {
                let (table, bytes) = TableType::from_bytes(bytes)?;
                Ok((Self::Table(table), bytes))
            }
            0x02 => {
                let (mem, bytes) = MemType::from_bytes(bytes)?;
                Ok((Self::Memory(mem), bytes))
            }
            0x03 => {
                let (global, bytes) = GlobalType::from_bytes(bytes)?;
                Ok((Self::Global(global), bytes))
            }
            x => Err(Error::ImportDescriptionTag(x)),
        }
    }
}
