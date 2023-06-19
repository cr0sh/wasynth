use super::{GlobalType, MemType, TableType};

#[derive(Clone, Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub description: ImportDescription,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportDescription {
    Func(u32),
    Table(TableType),
    Memory(MemType),
    Global(GlobalType),
}
