use crate::{
    context::{Context, IndexedCell},
    types::FuncType,
};

pub use crate::context::*;

/// A WebAssembly Module implementing its semantic structure.
///
/// https://webassembly.github.io/spec/core/syntax/modules.html
pub struct Module<'a> {
    pub context: &'a Context,
    pub start: Option<IndexedCell<'a, FuncType>>,
}
