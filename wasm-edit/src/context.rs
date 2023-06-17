use std::{any::type_name, cell::Cell, fmt::Debug};

use typed_arena::Arena;

use crate::types::{Data, Element, FuncType, Function, Global, MemType, TableType};

#[derive(Default)]
pub struct Context {
    types: Arena<Cell<FuncType>>,
    functions: Arena<Cell<Function>>,
    tables: Arena<Cell<TableType>>,
    memories: Arena<Cell<MemType>>,
    globals: Arena<Cell<Global>>,
    elements: Arena<Cell<Element>>,
    datas: Arena<Cell<Data>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_type(&self, ty: FuncType) -> IndexedRef<'_, FuncType> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.types.alloc(Cell::new(ty)),
        }
    }

    pub fn add_function(&self, function: Function) -> IndexedRef<'_, Function> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.functions.alloc(Cell::new(function)),
        }
    }

    pub fn add_table(&self, table: TableType) -> IndexedRef<'_, TableType> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.tables.alloc(Cell::new(table)),
        }
    }

    pub fn add_memory(&self, memory: MemType) -> IndexedRef<'_, MemType> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.memories.alloc(Cell::new(memory)),
        }
    }

    pub fn add_global(&self, global: Global) -> IndexedRef<'_, Global> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.globals.alloc(Cell::new(global)),
        }
    }

    pub fn add_element(&self, element: Element) -> IndexedRef<'_, Element> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.elements.alloc(Cell::new(element)),
        }
    }

    pub fn add_data(&self, data: Data) -> IndexedRef<'_, Data> {
        let index = self.types.len();
        IndexedRef {
            index,
            cell: self.datas.alloc(Cell::new(data)),
        }
    }
}

/// A symbolic reference to a WebAssembly type `T` which can obtain the index of itself.
#[derive(Clone, Copy)]
pub struct IndexedRef<'a, T> {
    index: usize,
    cell: &'a Cell<T>,
}

impl<'a, T> IndexedRef<'a, T> {
    /// Returns the index where this value is.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the index where this value is in `u32`.
    ///
    /// # Panics
    ///
    /// Panics if the index is larger than [`u32::MAX`].
    pub fn index_u32(&self) -> u32 {
        self.index().try_into().expect("index overflow")
    }
}

impl<'a, T> Debug for IndexedRef<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexedRef")
            .field("index", &self.index)
            .field("cell", &format!("Cell<{}>", type_name::<T>()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{ResultType, ValueType};

    use super::*;

    #[test]
    fn test_index() {
        let mut context = Context::new();
        let ty1 = context.add_type(FuncType {
            param: ResultType::new(&[]),
            result: ResultType::new(&[]),
        });
        let ty2 = context.add_type(FuncType {
            param: ResultType::new(&[ValueType::I32]),
            result: ResultType::new(&[ValueType::I64]),
        });

        assert_eq!(ty1.index(), 0);
        assert_eq!(ty2.index(), 1);
    }
}
