use std::io::{self, Write};

use crate::WriteExt;

#[derive(Clone, Debug, Default)]
pub struct SynthExportSection {
    pub(crate) exports: Vec<SynthExport>,
}

impl SynthExportSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        buf.write_vector(&self.exports, SynthExport::write_into)?;

        wr.write_all(&[7])?;
        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }

    pub fn exports(&self) -> &[SynthExport] {
        self.exports.as_ref()
    }

    pub fn exports_mut(&mut self) -> &mut Vec<SynthExport> {
        &mut self.exports
    }
}

#[derive(Clone, Debug)]
pub struct SynthExport {
    pub(crate) name: String,
    pub(crate) desc: SynthExportDescription,
}

impl SynthExport {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_name(&self.name)?;
        match self.desc {
            SynthExportDescription::Func(x) => {
                wr.write_all(&[0x00])?;
                wr.write_u32(x)?;
            }
            SynthExportDescription::Table(x) => {
                wr.write_all(&[0x01])?;
                wr.write_u32(x)?;
            }
            SynthExportDescription::Mem(x) => {
                wr.write_all(&[0x02])?;
                wr.write_u32(x)?;
            }
            SynthExportDescription::Global(x) => {
                wr.write_all(&[0x03])?;
                wr.write_u32(x)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum SynthExportDescription {
    Func(u32),
    Table(u32),
    Mem(u32),
    Global(u32),
}
