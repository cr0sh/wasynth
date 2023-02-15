use std::io::{self, Write};

use crate::{
    wasm_types::{GlobalType, MemType, TableType},
    WriteExt,
};

#[derive(Clone, Debug)]
pub struct SynthImportSection {
    pub(crate) imports: Vec<SynthImport>,
}

impl SynthImportSection {
    pub fn imports(&self) -> &[SynthImport] {
        self.imports.as_ref()
    }

    pub fn imports_mut(&mut self) -> &mut Vec<SynthImport> {
        &mut self.imports
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.imports, SynthImport::write_into)
    }
}

#[derive(Clone, Debug)]
pub struct SynthImport {
    pub(crate) module: String,
    pub(crate) name: String,
    pub(crate) description: SynthImportDescription,
}

impl SynthImport {
    pub fn module(&self) -> &str {
        self.module.as_ref()
    }

    pub fn module_mut(&mut self) -> &mut String {
        &mut self.module
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn description(&self) -> SynthImportDescription {
        self.description
    }

    pub fn description_mut(&mut self) -> &mut SynthImportDescription {
        &mut self.description
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_name(&self.module)?;
        wr.write_name(&self.name)?;
        self.description.write_into(wr)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynthImportDescription {
    Type(u32),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}

impl SynthImportDescription {
    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        match self {
            SynthImportDescription::Type(x) => {
                wr.write_all(&[0x00])?;
                wr.write_u32(*x)?;
                Ok(())
            }
            SynthImportDescription::Table(x) => {
                wr.write_all(&[0x00])?;
                x.write_into(wr)?;
                Ok(())
            }
            SynthImportDescription::Memory(x) => {
                wr.write_all(&[0x00])?;
                x.write_into(wr)?;
                Ok(())
            }
            SynthImportDescription::Global(x) => {
                wr.write_all(&[0x00])?;
                x.write_into(wr)?;
                Ok(())
            }
        }
    }
}
