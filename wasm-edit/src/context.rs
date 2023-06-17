use std::{any::type_name, cell::RefCell, fmt::Debug, marker::PhantomData, ops::Deref, rc::Rc};

use crate::types::{Data, Element, FuncType, Function, Global, MemType, TableType};

#[derive(Default)]
pub struct Context {
    types: Vec<Rc<RefCell<FuncType>>>,
    functions: Vec<Rc<RefCell<Function>>>,
    tables: Vec<Rc<RefCell<TableType>>>,
    memories: Vec<Rc<RefCell<MemType>>>,
    globals: Vec<Rc<RefCell<Global>>>,
    elements: Vec<Rc<RefCell<Element>>>,
    datas: Vec<Rc<RefCell<Data>>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_type(&self, ty: FuncType) -> IndexedCell<'_, FuncType> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(ty));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub fn add_function(&self, function: Function) -> IndexedCell<'_, Function> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(function));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub fn add_table(&self, table: TableType) -> IndexedCell<'_, TableType> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(table));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub fn add_memory(&self, memory: MemType) -> IndexedCell<'_, MemType> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(memory));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub fn add_global(&self, global: Global) -> IndexedCell<'_, Global> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(global));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub fn add_element(&self, element: Element) -> IndexedCell<'_, Element> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(element));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub fn add_data(&self, data: Data) -> IndexedCell<'_, Data> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(data));
        IndexedCell {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }
}

/// A symbolic reference to a WebAssembly type `T` which can obtain the index of itself.
#[derive(Clone)]
pub struct IndexedCell<'a, T> {
    index: usize,
    refcell: Rc<RefCell<T>>,
    _phantom: PhantomData<&'a ()>,
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
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.refcell
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{ResultType, ValueType};

    use super::*;

    #[test]
    fn test_index() {
        let context = Context::new();
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
