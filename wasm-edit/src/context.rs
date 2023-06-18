use std::{any::type_name, cell::RefCell, fmt::Debug, marker::PhantomData, ops::Deref, rc::Rc};

use crate::types::{Data, Element, FuncType, Function, Global, MemType, TableType};

#[derive(Default)]
pub struct Context {
    types: RefCell<Vec<Rc<RefCell<FuncType>>>>,
    functions: RefCell<Vec<Rc<RefCell<Function>>>>,
    tables: RefCell<Vec<Rc<RefCell<TableType>>>>,
    memories: RefCell<Vec<Rc<RefCell<MemType>>>>,
    globals: RefCell<Vec<Rc<RefCell<Global>>>>,
    elements: RefCell<Vec<Rc<RefCell<Element>>>>,
    datas: RefCell<Vec<Rc<RefCell<Data>>>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add_type(&self, ty: FuncType) -> IndexedRef<'_, FuncType> {
        let mut types = self.types.borrow_mut();
        let index = types.len();
        let refcell = Rc::new(RefCell::new(ty));
        types.push(Rc::clone(&refcell));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_function(&self, function: Function) -> IndexedRef<'_, Function> {
        let mut functions = self.functions.borrow_mut();
        let index = functions.len();
        let refcell = Rc::new(RefCell::new(function));
        functions.push(Rc::clone(&refcell));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_table(&self, table: TableType) -> IndexedRef<'_, TableType> {
        let mut tables = self.tables.borrow_mut();
        let index = tables.len();
        let refcell = Rc::new(RefCell::new(table));
        tables.push(Rc::clone(&refcell));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_memory(&self, memory: MemType) -> IndexedRef<'_, MemType> {
        let mut memories = self.memories.borrow_mut();
        let index = memories.len();
        let refcell = Rc::new(RefCell::new(memory));
        memories.push(Rc::clone(&refcell));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_global(&self, global: Global) -> IndexedRef<'_, Global> {
        let mut globals = self.globals.borrow_mut();
        let index = globals.len();
        let refcell = Rc::new(RefCell::new(global));
        globals.push(Rc::clone(&refcell));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_element(&self, element: Element) -> IndexedRef<'_, Element> {
        let mut elements = self.elements.borrow_mut();
        let index = elements.len();
        let refcell = Rc::new(RefCell::new(element));
        elements.push(Rc::clone(&refcell));
        IndexedRef {
            index,
            refcell,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_data(&self, data: Data) -> IndexedRef<'_, Data> {
        let mut datas = self.datas.borrow_mut();
        let index = datas.len();
        let refcell = Rc::new(RefCell::new(data));
        datas.push(Rc::clone(&refcell));
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
