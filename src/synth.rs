//! WebAssembly module 'synthesizer'.
//!
//! All types matching [`crate::parse`] should have `Synth` prefixes in its name.

use std::io::{self, Write};

use crate::{WriteExt, WASM_MAGIC, WASM_VERSION};

use self::sections::{
    SynthCodeSection, SynthCustomSection, SynthDataCountSection, SynthDataSection,
    SynthElementSection, SynthExportSection, SynthFunctionSection, SynthGlobalSection,
    SynthImportSection, SynthMemorySection, SynthStartSection, SynthTableSection, SynthTypeSection,
};

pub mod sections;

/// A WebAssembly module synthesizer.
pub struct SynthModule {
    pub(crate) sections: Vec<SynthSection>,
}

impl SynthModule {
    pub fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_all(WASM_MAGIC)?;
        wr.write_all(&WASM_VERSION.to_le_bytes())?;

        for section in &self.sections {
            section.write_into(wr)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum SynthSection {
    Custom(SynthCustomSection),
    Type(SynthTypeSection),
    Import(SynthImportSection),
    Function(SynthFunctionSection),
    Table(SynthTableSection),
    Memory(SynthMemorySection),
    Global(SynthGlobalSection),
    Export(SynthExportSection),
    Start(SynthStartSection),
    Element(SynthElementSection),
    Code(SynthCodeSection),
    Data(SynthDataSection),
    DataCount(SynthDataCountSection),
}

impl SynthSection {
    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_all(&[self.id()])?;

        let mut buf = Vec::new();
        match self {
            SynthSection::Custom(x) => x.write_into(&mut buf)?,
            SynthSection::Type(x) => x.write_into(&mut buf)?,
            SynthSection::Import(x) => x.write_into(&mut buf)?,
            SynthSection::Function(x) => x.write_into(&mut buf)?,
            SynthSection::Table(x) => x.write_into(&mut buf)?,
            SynthSection::Memory(x) => x.write_into(&mut buf)?,
            SynthSection::Global(x) => x.write_into(&mut buf)?,
            SynthSection::Export(x) => x.write_into(&mut buf)?,
            SynthSection::Start(x) => x.write_into(&mut buf)?,
            SynthSection::Element(x) => x.write_into(&mut buf)?,
            SynthSection::Code(x) => x.write_into(&mut buf)?,
            SynthSection::Data(x) => x.write_into(&mut buf)?,
            SynthSection::DataCount(x) => x.write_into(&mut buf)?,
        };

        wr.write_u32(buf.len().try_into().expect("buffer length overflow"))?;
        wr.write_all(&buf)?;

        Ok(())
    }

    /// Returns the ID of the section.
    pub fn id(&self) -> u8 {
        match self {
            Self::Custom(..) => 0,
            Self::Type(..) => 1,
            Self::Import(..) => 2,
            Self::Function(..) => 3,
            Self::Table(..) => 4,
            Self::Memory(..) => 5,
            Self::Global(..) => 6,
            Self::Export(..) => 7,
            Self::Start(..) => 8,
            Self::Element(..) => 9,
            Self::Code(..) => 10,
            Self::Data(..) => 11,
            Self::DataCount(..) => 12,
        }
    }
}
