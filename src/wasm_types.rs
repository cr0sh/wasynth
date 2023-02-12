//! WebAssembly (binary) types.
//!
//! <https://webassembly.github.io/spec/core/binary/types.html>

use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::{Bytes, Error, WriteExt};

/// A WebAssembly reference type.
///
/// <https://webassembly.github.io/spec/core/binary/types.html#reference-types>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceType {
    FuncRef,
    ExternRef,
}

impl ReferenceType {
    pub(crate) fn from_byte(byte: u8) -> Result<Self, Error> {
        match byte {
            0x70 => Ok(Self::FuncRef),
            0x6F => Ok(Self::ExternRef),
            x => Err(Error::ReferenceTypeId(x)),
        }
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        match self {
            ReferenceType::FuncRef => wr.write_all(&[0x70]),
            ReferenceType::ExternRef => wr.write_all(&[0x6F]),
        }
    }
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

impl ValueType {
    pub(crate) fn from_byte(byte: u8) -> Result<Self, Error> {
        match byte {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            0x7B => Ok(Self::V128),
            0x70 => Ok(Self::FuncRef),
            0x6F => Ok(Self::ExternRef),
            x => Err(Error::ValueTypeId(x)),
        }
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        match self {
            ValueType::I32 => wr.write_all(&[0x7F]),
            ValueType::I64 => wr.write_all(&[0x7E]),
            ValueType::F32 => wr.write_all(&[0x7D]),
            ValueType::F64 => wr.write_all(&[0x7C]),
            ValueType::V128 => wr.write_all(&[0x7B]),
            ValueType::FuncRef => wr.write_all(&[0x70]),
            ValueType::ExternRef => wr.write_all(&[0x6F]),
        }
    }
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
pub struct ResultType(Vec<ValueType>);

impl ResultType {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let mut v = Vec::new();
        let mut it = bytes.advance_vector(|bytes| {
            let (&[b], bytes) = bytes.advance()?;
            Ok((ValueType::from_byte(b)?, bytes))
        })?;
        for t in &mut it {
            v.push(t?)
        }

        Ok((Self(v), it.finalize()))
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_vector(&self.0, ValueType::write_into)
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
    param: ResultType,
    result: ResultType,
}

impl FuncType {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[id], bytes) = bytes.advance()?;
        if id != 0x60 {
            return Err(Error::FunctionTypeId(id));
        }
        let (rt1, bytes) = ResultType::from_bytes(bytes)?;
        let (rt2, bytes) = ResultType::from_bytes(bytes)?;

        Ok((
            Self {
                param: rt1,
                result: rt2,
            },
            bytes,
        ))
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_all(&[0x60])?;
        self.param.write_into(wr)?;
        self.result.write_into(wr)?;
        Ok(())
    }

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

impl Limits {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[id], bytes) = bytes.advance()?;
        match id {
            0x00 => {
                let (min, bytes) = bytes.advance_u32()?;
                Ok((Self::Unbounded { min }, bytes))
            }
            0x01 => {
                let (min, bytes) = bytes.advance_u32()?;
                let (max, bytes) = bytes.advance_u32()?;
                Ok((Self::Bounded { min, max }, bytes))
            }
            x => Err(Error::LimitsTag(x)),
        }
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        match *self {
            Limits::Unbounded { min } => {
                wr.write_all(&[0x00])?;
                wr.write_u32(min)?;
                Ok(())
            }
            Limits::Bounded { min, max } => {
                wr.write_all(&[0x01])?;
                wr.write_u32(min)?;
                wr.write_u32(max)?;
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemType {
    size: Limits,
}

impl MemType {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        Limits::from_bytes(bytes).map(|(l, bytes)| (Self { size: l }, bytes))
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        self.size.write_into(wr)
    }

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
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[element], bytes) = bytes.advance()?;
        let element = ReferenceType::from_byte(element)?;
        let (limits, bytes) = Limits::from_bytes(bytes)?;
        Ok((Self { element, limits }, bytes))
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        self.element.write_into(&mut wr)?;
        self.limits.write_into(&mut wr)?;
        Ok(())
    }

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

impl GlobalType {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[ty], bytes) = bytes.advance()?;
        let ty = ValueType::from_byte(ty)?;
        let (&[mutable], bytes) = bytes.advance()?;
        let mutable = match mutable {
            0x00 => false,
            0x01 => true,
            x => return Err(Error::GlobalTypeMutability(x)),
        };

        Ok((Self { ty, mutable }, bytes))
    }

    pub(crate) fn write_into(&self, mut wr: &mut impl Write) -> Result<(), io::Error> {
        self.ty.write_into(&mut wr)?;
        wr.write_all(&[u8::from(self.mutable)])?;
        Ok(())
    }
}
