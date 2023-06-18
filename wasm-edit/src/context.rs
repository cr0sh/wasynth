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

    pub(crate) fn add_type(&self, ty: FuncType) -> IndexedRef<'_, FuncType> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(ty));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_function(&self, function: Function) -> IndexedRef<'_, Function> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(function));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_table(&self, table: TableType) -> IndexedRef<'_, TableType> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(table));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_memory(&self, memory: MemType) -> IndexedRef<'_, MemType> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(memory));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_global(&self, global: Global) -> IndexedRef<'_, Global> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(global));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_element(&self, element: Element) -> IndexedRef<'_, Element> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(element));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_data(&self, data: Data) -> IndexedRef<'_, Data> {
        let index = self.types.len();
        let refcell = Rc::new(RefCell::new(data));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }
}

/// A symbolic reference to a WebAssembly type `T` which can obtain the index of itself.
#[derive(Clone)]
pub struct IndexedRef<'a, T> {
    index: usize,
    refcell: Rc<RefCell<T>>,
    _phantom: PhantomData<&'a ()>,
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

    /// Returns if this reference is unused by any other entities.
    ///
    /// Basically this returns true iff the strong refcount of the inner [`Rc`] is 1.
    pub(crate) fn is_unused(&self) -> bool {
        Rc::strong_count(&self.refcell) == 1
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

impl<'a, T> Deref for IndexedRef<'a, T> {
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
