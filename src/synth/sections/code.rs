use crate::{instructions::Expression, wasm_types::ValueType};

#[derive(Clone, Debug)]
pub struct CodeSection {
    pub(crate) codes: Vec<Code>,
}

impl CodeSection {
    pub fn codes(&self) -> &[Code] {
        self.codes.as_ref()
    }

    pub fn codes_mut(&mut self) -> &mut Vec<Code> {
        &mut self.codes
    }
}

#[derive(Clone, Debug)]
pub struct Code {
    locals: Vec<ValueType>,
    func_expr: Expression,
}

impl Code {
    pub fn func_expr(&self) -> &Expression {
        &self.func_expr
    }
}
