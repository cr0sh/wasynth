//! WebAssembly (binary) types.
//!
//! <https://webassembly.github.io/spec/core/binary/types.html>

// section-specific types
mod custom;
mod element;
mod export;
mod import;

use std::fmt::Display;

use arrayvec::ArrayVec;

pub use custom::*;
pub use element::*;
pub use export::*;
pub use import::*;

/// Max capacity of [`ValueType`]s that a single [`ResultType`] can hold.
pub const RESULT_TYPE_ARRAY_MAX_SIZE: usize = 256;

/// A WebAssembly reference type.
///
/// <https://webassembly.github.io/spec/core/binary/types.html#reference-types>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceType {
    FuncRef,
    ExternRef,
}

impl Display for ReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferenceType::FuncRef => write!(f, "funcref"),
            ReferenceType::ExternRef => write!(f, "externref"),
        }
    }
}

impl From<ReferenceType> for ValueType {
    fn from(value: ReferenceType) -> Self {
        match value {
            ReferenceType::FuncRef => ValueType::FuncRef,
            ReferenceType::ExternRef => ValueType::ExternRef,
        }
    }
}

/// A WebAssembly value type.
///
/// <https://webassembly.github.io/spec/core/binary/types.html#value-types>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueType {
    // NOTE: this is not a Rust i32, but WebAssembly 32-bit wide uninterpreted 'integer'.
    I32,
    // NOTE: this is not a Rust i64, but WebAssembly 64-bit wide uninterpreted 'integer'.
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::I32 => write!(f, "i32"),
            ValueType::I64 => write!(f, "i64"),
            ValueType::F32 => write!(f, "f32"),
            ValueType::F64 => write!(f, "f64"),
            ValueType::V128 => write!(f, "v128"),
            ValueType::FuncRef => write!(f, "funcref"),
            ValueType::ExternRef => write!(f, "externref"),
        }
    }
}

/// A WebAssembly result type.
///
/// <https://webassembly.github.io/spec/core/binary/types.html#result-types>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResultType(ArrayVec<ValueType, RESULT_TYPE_ARRAY_MAX_SIZE>);

impl ResultType {
    pub fn new(types: &[ValueType]) -> Self {
        let mut v = ArrayVec::new();
        for ty in types {
            v.push(*ty);
        }
        Self(v)
    }

    pub fn types(&self) -> &[ValueType] {
        &self.0
    }

    pub fn types_mut(&mut self) -> &mut ArrayVec<ValueType, RESULT_TYPE_ARRAY_MAX_SIZE> {
        &mut self.0
    }
}

impl Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "({inner})")
    }
}

/// A WebAssembly function type.
///
/// <https://webassembly.github.io/spec/core/binary/types.html#function-types>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuncType {
    pub(crate) param: ResultType,
    pub(crate) result: ResultType,
}

impl FuncType {
    pub fn param(&self) -> &ResultType {
        &self.param
    }

    pub fn result(&self) -> &ResultType {
        &self.result
    }
}

impl Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.param, self.result)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Limits {
    Unbounded { min: u32 },
    Bounded { min: u32, max: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemType {
    size: Limits,
}

impl MemType {
    pub fn size(&self) -> &Limits {
        &self.size
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableType {
    element: ReferenceType,
    limits: Limits,
}

impl TableType {
    pub fn element(&self) -> ReferenceType {
        self.element
    }

    pub fn limits(&self) -> &Limits {
        &self.limits
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlobalType {
    ty: ValueType,
    mutable: bool,
}
