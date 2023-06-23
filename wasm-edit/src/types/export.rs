use crate::context::IndexedRef;

use super::{Function, Global, MemType, TableType};

pub struct Export<'a> {
    pub name: String,
    pub description: ExportDescription<'a>,
}

#[derive(Clone, Debug)]
pub enum ExportDescription<'a> {
    Func(IndexedRef<'a, Function>),
    Table(IndexedRef<'a, TableType>),
    Mem(IndexedRef<'a, MemType>),
    Global(IndexedRef<'a, Global>),
}
