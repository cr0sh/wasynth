use crate::{
    context::{Context, IndexedRef},
    types::{Data, Element, FuncType, Function, Global, MemType, TableType},
};

pub use crate::context::*;

/// A WebAssembly Module implementing its semantic structure.
///
/// <https://webassembly.github.io/spec/core/syntax/modules.html>
pub struct Module<'a> {
    context: &'a Context,
    pub start: Option<IndexedRef<'a, FuncType>>,
}

impl<'a> Module<'a> {
    pub fn from_context(context: &'a mut Context) -> Self {
        Self {
            context,
            start: None,
        }
    }

    /// Add a [`FuncType`] value to this Module. Returns the reference to the added value.
    pub fn add_type(&self, ty: FuncType) -> IndexedRef<'_, FuncType> {
        self.context.add_type(ty)
    }

    /// Add a [`Function`] value to this Module. Returns the reference to the added value.
    pub fn add_function(&self, function: Function) -> IndexedRef<'_, Function> {
        self.context.add_function(function)
    }

    /// Add a [`TableType`] value to this Module. Returns the reference to the added value.
    pub fn add_table(&self, table: TableType) -> IndexedRef<'_, TableType> {
        self.context.add_table(table)
    }

    /// Add a [`MemType`] value to this Module. Returns the reference to the added value.
    pub fn add_memory(&self, memory: MemType) -> IndexedRef<'_, MemType> {
        self.context.add_memory(memory)
    }

    /// Add a [`Global`] value to this Module. Returns the reference to the added value.
    pub fn add_global(&self, global: Global) -> IndexedRef<'_, Global> {
        self.context.add_global(global)
    }

    /// Add an [`Element`] value to this Module. Returns the reference to the added value.
    pub fn add_element(&self, element: Element) -> IndexedRef<'_, Element> {
        self.context.add_element(element)
    }

    /// Add a [`Data`] value to this Module. Returns the reference to the added value.
    pub fn add_data(&self, data: Data) -> IndexedRef<'_, Data> {
        self.context.add_data(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{ResultType, ValueType};

    use super::*;

    #[test]
    fn test_index() {
        let mut context = Context::new();
        let module = Module::from_context(&mut context);
        let ty1 = module.add_type(FuncType {
            param: ResultType::new(&[]),
            result: ResultType::new(&[]),
        });
        let ty2 = module.add_type(FuncType {
            param: ResultType::new(&[ValueType::I32]),
            result: ResultType::new(&[ValueType::I64]),
        });

        assert_eq!(ty1.index(), 0);
        assert_eq!(ty2.index(), 1);
    }
}
