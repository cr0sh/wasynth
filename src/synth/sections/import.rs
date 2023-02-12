use std::io::{self, Write};

use crate::{
    wasm_types::{GlobalType, MemType, TableType},
    WriteExt,
};

#[derive(Clone, Debug)]
pub struct ImportSection {
    pub(crate) imports: Vec<Import>,
}

impl ImportSection {
    pub fn imports(&self) -> &[Import] {
        self.imports.as_ref()
    }

    pub fn imports_mut(&mut self) -> &mut Vec<Import> {
        &mut self.imports
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.imports, Import::write_into)
    }
}

#[derive(Clone, Debug)]
pub struct Import {
    pub(crate) module: String,
    pub(crate) name: String,
    pub(crate) description: ImportDescription,
}

impl Import {
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

    pub fn description(&self) -> ImportDescription {
        self.description
    }

    pub fn description_mut(&mut self) -> &mut ImportDescription {
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
pub enum ImportDescription {
    Type(u32),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}

impl ImportDescription {
    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        match self {
            ImportDescription::Type(x) => {
                wr.write_all(&[0x00])?;
                wr.write_u32(*x)?;
                Ok(())
            }
            ImportDescription::Table(x) => {
                wr.write_all(&[0x00])?;
                x.write_into(wr)?;
                Ok(())
            }
            ImportDescription::Memory(x) => {
                wr.write_all(&[0x00])?;
                x.write_into(wr)?;
                Ok(())
            }
            ImportDescription::Global(x) => {
                wr.write_all(&[0x00])?;
                x.write_into(wr)?;
                Ok(())
            }
        }
    }
}
