use std::cell::{Cell, RefCell};

use crate::{
    context::{Context, IndexedRef},
    types::{Data, Element, Export, FuncType, Function, Global, MemType, TableType},
};

pub use crate::context::*;

/// A WebAssembly Module implementing its semantic structure.
///
/// <https://webassembly.github.io/spec/core/syntax/modules.html>
pub struct Module<'a> {
    context: &'a Context,
    start: Option<IndexedRef<'a, FuncType>>,
    exports: RefCell<Vec<Export<'a>>>,
}

impl<'a> Module<'a> {
    pub fn from_context(context: &'a mut Context) -> Self {
        Self {
            context,
            start: None,
            exports: RefCell::new(Vec::new()),
        }
    }

    /// Add a [`FuncType`] value to this Module. Returns the reference to the added value.
    pub fn add_type(&self, ty: FuncType) -> IndexedRef<'_, FuncType> {
        self.context.add_type(ty)
    }

    /// Add a [`Function`] value to this Module. Returns the reference to the added value.
    pub fn add_function(&self, function: Function) -> ImportOrStandalone<'_, Function> {
        ImportOrStandalone::Standalone {
            preceding_imports: &self.context.num_function_imports,
            indexed_ref: self.context.add_function(function),
        }
    }

    /// Add a [`TableType`] value to this Module. Returns the reference to the added value.
    pub fn add_table(&self, table: TableType) -> ImportOrStandalone<'_, TableType> {
        ImportOrStandalone::Standalone {
            preceding_imports: &self.context.num_table_imports,
            indexed_ref: self.context.add_table(table),
        }
    }

    /// Add a [`MemType`] value to this Module. Returns the reference to the added value.
    pub fn add_memory(&self, memory: MemType) -> ImportOrStandalone<'_, MemType> {
        ImportOrStandalone::Standalone {
            preceding_imports: &self.context.num_memory_imports,
            indexed_ref: self.context.add_memory(memory),
        }
    }

    /// Add a [`Global`] value to this Module. Returns the reference to the added value.
    pub fn add_global(&self, global: Global) -> ImportOrStandalone<'_, Global> {
        ImportOrStandalone::Standalone {
            preceding_imports: &self.context.num_global_imports,
            indexed_ref: self.context.add_global(global),
        }
    }

    /// Add an [`Element`] value to this Module. Returns the reference to the added value.
    pub fn add_element(&self, element: Element) -> IndexedRef<'_, Element> {
        self.context.add_element(element)
    }

    /// Add a [`Data`] value to this Module. Returns the reference to the added value.
    pub fn add_data(&self, data: Data) -> IndexedRef<'_, Data> {
        self.context.add_data(data)
    }

    /// Add a function import to this module. Returns the reference to the imported function.
    pub fn import_function(
        &self,
        module: String,
        name: String,
        ty: FuncType,
    ) -> ImportOrStandalone<'_, FuncType> {
        ImportOrStandalone::Import {
            module,
            name,
            indexed_ref: self.context.add_function_import(ty),
        }
    }

    /// Add a table import to this module. Returns the reference to the imported function.
    pub fn import_table(
        &self,
        module: String,
        name: String,
        table: TableType,
    ) -> ImportOrStandalone<'_, TableType> {
        ImportOrStandalone::Import {
            module,
            name,
            indexed_ref: self.context.add_table_import(table),
        }
    }

    /// Add a memory import to this module. Returns the reference to the imported function.
    pub fn import_memory(
        &self,
        module: String,
        name: String,
        memory: MemType,
    ) -> ImportOrStandalone<'_, MemType> {
        ImportOrStandalone::Import {
            module,
            name,
            indexed_ref: self.context.add_memory_import(memory),
        }
    }

    /// Add a global import to this module. Returns the reference to the imported function.
    pub fn import_global(
        &self,
        module: String,
        name: String,
        global: Global,
    ) -> ImportOrStandalone<'_, Global> {
        ImportOrStandalone::Import {
            module,
            name,
            indexed_ref: self.context.add_global_import(global),
        }
    }

    pub fn export_function(&'a self, name: String, function: IndexedRef<'a, Function>) {
        self.exports.borrow_mut().push(Export {
            name,
            description: crate::types::ExportDescription::Func(function),
        });
    }

    pub fn export_table(&'a self, name: String, table: IndexedRef<'a, TableType>) {
        self.exports.borrow_mut().push(Export {
            name,
            description: crate::types::ExportDescription::Table(table),
        });
    }

    pub fn export_memory(&'a self, name: String, memory: IndexedRef<'a, MemType>) {
        self.exports.borrow_mut().push(Export {
            name,
            description: crate::types::ExportDescription::Mem(memory),
        });
    }

    pub fn export_global(&'a self, name: String, global: IndexedRef<'a, Global>) {
        self.exports.borrow_mut().push(Export {
            name,
            description: crate::types::ExportDescription::Global(global),
        });
    }
}

pub enum ImportOrStandalone<'a, T> {
    Import {
        module: String,
        name: String,
        indexed_ref: IndexedRef<'a, T>,
    },
    Standalone {
        preceding_imports: &'a Cell<usize>,
        indexed_ref: IndexedRef<'a, T>,
    },
}

#[cfg(test)]
mod tests {
    use crate::{
        instructions::Expression,
        types::{ResultType, ValueType},
    };

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

    #[test]
    fn test_num_imports() {
        let mut context = Context::new();
        let module = Module::from_context(&mut context);
        let func1 = module.add_function(Function::new(
            FuncType {
                param: ResultType::new(&[ValueType::I32]),
                result: ResultType::new(&[ValueType::I64]),
            },
            &[],
            Expression(vec![]),
        ));

        match func1 {
            ImportOrStandalone::Import { .. } => panic!("match failed"),
            ImportOrStandalone::Standalone {
                preceding_imports, ..
            } => assert_eq!(preceding_imports.get(), 0),
        }

        let _import1 = module.import_function(
            String::from("hello"),
            String::from("world"),
            FuncType {
                param: ResultType::new(&[]),
                result: ResultType::new(&[]),
            },
        );

        match func1 {
            ImportOrStandalone::Import { .. } => panic!("match failed"),
            ImportOrStandalone::Standalone {
                preceding_imports, ..
            } => assert_eq!(preceding_imports.get(), 1),
        }
    }
}
