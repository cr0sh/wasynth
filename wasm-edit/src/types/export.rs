use crate::context::IndexedRef;

use super::{FuncType, TableType};

pub struct Export<'a> {
    pub name: String,
    pub description: ExportDescription<'a>,
}

#[derive(Clone, Debug)]
pub enum ExportDescription<'a> {
    Func(IndexedRef<'a, FuncType>),
    Table(IndexedRef<'a, TableType>),
    Mem(u32),
    Global(u32),
}
