use crate::{
    context::{Context, IndexedCell},
    types::{Data, Element, FuncType, Function, Global, MemType, TableType},
};

pub use crate::context::*;

/// A WebAssembly Module implementing its semantic structure.
///
/// https://webassembly.github.io/spec/core/syntax/modules.html
pub struct Module<'a> {
    context: &'a Context,
    pub start: Option<IndexedCell<'a, FuncType>>,
}

impl<'a> Module<'a> {
    /// Add a [`FuncType`] value to this Module. Returns the reference to the added value.
    pub fn add_type(&self, ty: FuncType) -> IndexedCell<'_, FuncType> {
        self.context.add_type(ty)
    }

    /// Add a [`Function`] value to this Module. Returns the reference to the added value.
    pub fn add_function(&self, function: Function) -> IndexedCell<'_, Function> {
        self.context.add_function(function)
    }

    /// Add a [`TableType`] value to this Module. Returns the reference to the added value.
    pub fn add_table(&self, table: TableType) -> IndexedCell<'_, TableType> {
        self.context.add_table(table)
    }

    /// Add a [`MemType`] value to this Module. Returns the reference to the added value.
    pub fn add_memory(&self, memory: MemType) -> IndexedCell<'_, MemType> {
        self.context.add_memory(memory)
    }

    /// Add a [`Global`] value to this Module. Returns the reference to the added value.
    pub fn add_global(&self, global: Global) -> IndexedCell<'_, Global> {
        self.context.add_global(global)
    }

    /// Add an [`Element`] value to this Module. Returns the reference to the added value.
    pub fn add_element(&self, element: Element) -> IndexedCell<'_, Element> {
        self.context.add_element(element)
    }

    /// Add a [`Data`] value to this Module. Returns the reference to the added value.
    pub fn add_data(&self, data: Data) -> IndexedCell<'_, Data> {
        self.context.add_data(data)
    }
}
