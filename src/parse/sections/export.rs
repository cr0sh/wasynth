use std::fmt::Debug;

use crate::{
    synth::sections::{SynthExport, SynthExportDescription, SynthExportSection},
    Bytes, Error,
};

#[derive(Clone, Copy)]
pub struct ExportSection<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> ExportSection<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<Self, Error> {
        Ok(Self { bytes })
    }

    pub(crate) fn into_synth(self) -> Result<SynthExportSection, Error> {
        Ok(SynthExportSection {
            exports: self
                .exports()?
                .map(|x| x.map(Export::into_synth))
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }

    pub fn exports(
        &self,
    ) -> Result<impl Iterator<Item = Result<Export<'bytes>, Error>> + '_, Error> {
        self.bytes.advance_vector(Export::from_bytes)
    }
}

impl<'bytes> Debug for ExportSection<'bytes> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExportSection").finish()
    }
}

#[derive(Clone, Debug)]
pub struct Export<'bytes> {
    pub(crate) name: &'bytes str,
    pub(crate) desc: ExportDescription,
}

impl<'bytes> Export<'bytes> {
    pub(crate) fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &'bytes [u8]), Error> {
        let (name, bytes) = bytes.advance_name()?;
        let (&[discriminator], bytes) = bytes.advance()?;
        let (idx, bytes) = bytes.advance_u32()?;
        let desc = match discriminator {
            0x00 => ExportDescription::Func(idx),
            0x01 => ExportDescription::Table(idx),
            0x02 => ExportDescription::Mem(idx),
            0x03 => ExportDescription::Global(idx),
            other => return Err(Error::ExportDescription(other)),
        };

        Ok((Self { name, desc }, bytes))
    }

    pub(crate) fn into_synth(self) -> SynthExport {
        SynthExport {
            name: self.name.to_string(),
            desc: match self.desc {
                ExportDescription::Func(x) => SynthExportDescription::Func(x),
                ExportDescription::Table(x) => SynthExportDescription::Table(x),
                ExportDescription::Mem(x) => SynthExportDescription::Mem(x),
                ExportDescription::Global(x) => SynthExportDescription::Global(x),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExportDescription {
    Func(u32),
    Table(u32),
    Mem(u32),
    Global(u32),
}
