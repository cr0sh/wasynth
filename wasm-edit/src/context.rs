use std::{any::type_name, cell::Cell, fmt::Debug, ops::Deref};

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

    pub fn add_type(&self, ty: FuncType) -> IndexedCell<'_, FuncType> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.types.alloc(Cell::new(ty)),
        }
    }

    pub fn add_function(&self, function: Function) -> IndexedCell<'_, Function> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.functions.alloc(Cell::new(function)),
        }
    }

    pub fn add_table(&self, table: TableType) -> IndexedCell<'_, TableType> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.tables.alloc(Cell::new(table)),
        }
    }

    pub fn add_memory(&self, memory: MemType) -> IndexedCell<'_, MemType> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.memories.alloc(Cell::new(memory)),
        }
    }

    pub fn add_global(&self, global: Global) -> IndexedCell<'_, Global> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.globals.alloc(Cell::new(global)),
        }
    }

    pub fn add_element(&self, element: Element) -> IndexedCell<'_, Element> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.elements.alloc(Cell::new(element)),
        }
    }

    pub fn add_data(&self, data: Data) -> IndexedCell<'_, Data> {
        let index = self.types.len();
        IndexedCell {
            index,
            cell: self.datas.alloc(Cell::new(data)),
        }
    }
}

/// A symbolic reference to a WebAssembly type `T` which can obtain the index of itself.
#[derive(Clone, Copy)]
pub struct IndexedCell<'a, T> {
    index: usize,
    cell: &'a Cell<T>,
}

impl<'a, T> IndexedCell<'a, T> {
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

impl<'a, T> Debug for IndexedCell<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexedRef")
            .field("index", &self.index)
            .field("cell", &format!("Cell<{}>", type_name::<T>()))
            .finish()
    }
}

impl<'a, T> Deref for IndexedCell<'a, T> {
    type Target = Cell<T>;

    fn deref(&self) -> &Self::Target {
        self.cell
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
