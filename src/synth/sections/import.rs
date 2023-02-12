use crate::wasm_types::{GlobalType, MemType, TableType};

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
}

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportDescription {
    Type(u32),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}
