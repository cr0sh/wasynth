//! WebAssembly modules parser.
//!
//! <https://webassembly.github.io/spec/core/binary/modules.html>

pub mod sections;

use std::fmt::Debug;

use crate::{
    synth::{SynthModule, SynthSection},
    Bytes, Error, WASM_MAGIC, WASM_VERSION,
};
use log::trace;
use sections::{
    CodeSection, CustomSection, DataCountSection, DataSection, ElementSection, ExportSection,
    FunctionSection, GlobalSection, ImportSection, MemorySection, StartSection, TableSection,
    TypeSection,
};

/// A parsed WebAssembly module.
#[derive(Debug, Clone)]
pub struct Module<'bytes> {
    sections: Vec<Section<'bytes>>,
}

impl<'bytes> Module<'bytes> {
    pub fn from_binary(binary: &'bytes [u8]) -> Result<Self, Error> {
        #[cfg(bytes_trace)]
        {
            crate::bytes_trace::initialize(binary);
        }

        let (magic, binary) = binary.advance::<4>()?;
        if magic != WASM_MAGIC {
            return Err(Error::Magic(magic[0], magic[1], magic[2], magic[3]));
        }

        let (version, mut binary) = binary.advance()?;
        let version = u32::from_le_bytes(*version);
        if version != WASM_VERSION {
            return Err(Error::UnsupportedVersion(version));
        }

        let mut sections = Vec::new();

        while !binary.is_empty() {
            trace!("start reading section");
            let (section, rest) = Section::from_bytes(binary)?;
            trace!("end reading section, id={}", section.id());
            binary = rest;
            sections.push(section);
        }

        if !binary.is_empty() {
            return Err(Error::TrailingBytes);
        }

        Ok(Module { sections })
    }

    pub fn into_synth(self) -> Result<SynthModule, Error> {
        Ok(SynthModule {
            sections: self
                .sections
                .into_iter()
                .map(Section::into_synth)
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }

    pub fn sections(&self) -> &[Section<'bytes>] {
        &self.sections
    }

    pub fn validate(&self) -> Result<(), Error> {
        for section in self.sections() {
            match section {
                Section::Custom(_) => (),
                Section::Type(s) => {
                    for ty in s.types()? {
                        ty?;
                    }
                }
                Section::Import(s) => {
                    for im in s.imports()? {
                        im?;
                    }
                }
                Section::Function(s) => {
                    for tyidx in s.type_indices()? {
                        tyidx?;
                    }
                }
                Section::Table(s) => {
                    for table in s.tables()? {
                        table?;
                    }
                }
                Section::Memory(s) => {
                    for mem in s.memories()? {
                        mem?;
                    }
                }
                Section::Global(_) => (),
                Section::Export(_) => (),
                Section::Start(_) => (),
                Section::Element(_) => (),
                Section::Code(s) => {
                    for code in s.codes()? {
                        code?;
                    }
                }
                Section::Data(s) => {
                    for data in s.all_data()? {
                        data?;
                    }
                }
                Section::DataCount(_) => (),
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Section<'bytes> {
    Custom(CustomSection<'bytes>),
    Type(TypeSection<'bytes>),
    Import(ImportSection<'bytes>),
    Function(FunctionSection<'bytes>),
    Table(TableSection<'bytes>),
    Memory(MemorySection<'bytes>),
    Global(GlobalSection<'bytes>),
    Export(ExportSection<'bytes>),
    Start(StartSection<'bytes>),
    Element(ElementSection<'bytes>),
    Code(CodeSection<'bytes>),
    Data(DataSection<'bytes>),
    DataCount(DataCountSection),
}

impl<'bytes> Section<'bytes> {
    fn from_bytes(bytes: &'bytes [u8]) -> Result<(Self, &'bytes [u8]), Error> {
        let (&[id], bytes) = bytes.advance()?;
        let (len, bytes) = bytes.advance_u32()?;
        let (bytes, rest) = bytes.advance_slice(len.try_into().expect("section size overflow"))?;

        let section = match id {
            0 => Self::Custom(CustomSection::from_bytes(bytes)?),
            1 => Self::Type(TypeSection::from_bytes(bytes)?),
            2 => Self::Import(ImportSection::from_bytes(bytes)?),
            3 => Self::Function(FunctionSection::from_bytes(bytes)?),
            4 => Self::Table(TableSection::from_bytes(bytes)?),
            5 => Self::Memory(MemorySection::from_bytes(bytes)?),
            6 => Self::Global(GlobalSection::from_bytes(bytes)?),
            7 => Self::Export(ExportSection::from_bytes(bytes)?),
            8 => Self::Start(StartSection::from_bytes(bytes)?),
            9 => Self::Element(ElementSection::from_bytes(bytes)?),
            10 => Self::Code(CodeSection::from_bytes(bytes)?),
            11 => Self::Data(DataSection::from_bytes(bytes)?),
            12 => Self::DataCount(DataCountSection::from_bytes(bytes)?),
            x => return Err(Error::SectionID(x)),
        };

        Ok((section, rest))
    }

    pub fn into_synth(self) -> Result<SynthSection, Error> {
        match self {
            Section::Custom(x) => Ok(SynthSection::Custom(x.into_synth())),
            Section::Type(x) => Ok(SynthSection::Type(x.into_synth()?)),
            Section::Import(x) => Ok(SynthSection::Import(x.into_synth()?)),
            Section::Function(x) => Ok(SynthSection::Function(x.into_synth()?)),
            Section::Table(x) => Ok(SynthSection::Table(x.into_synth()?)),
            Section::Memory(x) => Ok(SynthSection::Memory(x.into_synth()?)),
            Section::Global(x) => Ok(SynthSection::Global(x.into_synth())),
            Section::Export(x) => Ok(SynthSection::Export(x.into_synth())),
            Section::Start(x) => Ok(SynthSection::Start(x.into_synth())),
            Section::Element(x) => Ok(SynthSection::Element(x.into_synth())),
            Section::Code(x) => Ok(SynthSection::Code(x.into_synth()?)),
            Section::Data(x) => Ok(SynthSection::Data(x.into_synth()?)),
            Section::DataCount(x) => Ok(SynthSection::DataCount(x.into_synth())),
        }
    }

    /// Returns the ID of the section.
    pub fn id(self) -> u8 {
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
