use crate::instructions::Expression;

use super::{FuncType, ValueType};

pub struct Function {
    pub ty: FuncType,
    locals: Vec<ValueType>,
    pub body: Expression,
}

impl Function {
    pub fn new(ty: FuncType, locals: &[ValueType], body: Expression) -> Self {
        let mut locals_vec = Vec::new();
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

    pub fn locals_mut(&mut self) -> &mut Vec<ValueType> {
        &mut self.locals
    }
}
