use crate::instructions::Expression;

use super::ReferenceType;

#[derive(Clone, Debug)]
pub struct Element {
    pub kind: ElementKind,
    pub init: ElementInit,
    pub mode: ElementMode,
}

#[derive(Clone, Copy, Debug)]
pub enum ElementKind {
    FuncRef,
    // For the sake of simplicity, we do not create ElemKindOrRefType enum so put that case here
    ReferenceType(ReferenceType),
}

#[derive(Clone, Debug)]
pub enum ElementInit {
    FuncIndices(Vec<u32>),
    Expressions(Vec<Expression>),
}

#[derive(Clone, Debug)]
pub enum ElementMode {
    Active { table: u32, offset: Expression },
    Passive,
    Declarative,
}
