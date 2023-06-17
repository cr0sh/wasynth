use crate::context::IndexedCell;

use super::{FuncType, TableType};

pub struct Export<'a> {
    pub name: String,
    pub description: ExportDescription<'a>,
}

#[derive(Clone, Debug)]
pub enum ExportDescription<'a> {
    Func(IndexedCell<'a, FuncType>),
    Table(IndexedCell<'a, TableType>),
    Mem(u32),
    Global(u32),
}
