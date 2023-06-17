use arrayvec::ArrayVec;

use crate::{instructions::Expression, types::RESULT_TYPE_ARRAY_MAX_SIZE};

use super::{FuncType, ValueType};

pub struct Function {
    pub ty: FuncType,
    locals: ArrayVec<ValueType, RESULT_TYPE_ARRAY_MAX_SIZE>,
    pub body: Expression,
}

impl Function {
    pub fn new(ty: FuncType, locals: &[ValueType], body: Expression) -> Self {
        let mut locals_vec = ArrayVec::new();
        for local in locals {
            locals_vec.push(*local);
        }
        Self {
            ty,
            locals: locals_vec,
            body,
        }
    }

    pub fn locals(&self) -> &[ValueType] {
        &self.locals
    }

    pub fn locals_mut(&mut self) -> &mut ArrayVec<ValueType, RESULT_TYPE_ARRAY_MAX_SIZE> {
        &mut self.locals
    }
}
