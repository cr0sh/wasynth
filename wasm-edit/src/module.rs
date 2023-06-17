use crate::{
    context::{Context, IndexedRef},
    types::FuncType,
};

pub use crate::context::*;

/// A WebAssembly Module implementing its semantic structure.
///
/// https://webassembly.github.io/spec/core/syntax/modules.html
pub struct Module<'a> {
    context: &'a Context,
    start: Option<IndexedRef<'a, FuncType>>,
}
