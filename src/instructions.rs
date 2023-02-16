use std::io::{self, Write};

use crate::{
    wasm_types::{ReferenceType, ValueType},
    Bytes, Error, WriteExt,
};

#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    // NOTE: The specification defines the type index as s33
    TypeIndex(i64),
}

#[allow(dead_code)]
impl BlockType {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        if bytes.is_empty() {
            return Err(Error::UnexpectedEof(1, 0));
        }
        if bytes[0] == 0x40 {
            return Ok((Self::Empty, &bytes[1..]));
        }

        match ValueType::from_byte(bytes[0]) {
            Ok(x) => Ok((Self::Value(x), &bytes[1..])),
            Err(Error::ValueTypeId(..)) => {
                let (tyidx, bytes) = bytes.advance_s64()?;
                Ok((Self::TypeIndex(tyidx), bytes))
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        match *self {
            BlockType::Empty => wr.write_all(&[0x40]),
            BlockType::Value(vt) => vt.write_into(wr),
            BlockType::TypeIndex(x) => {
                if !(-(1i64 << 32)..=((1i64 << 32) - 1)).contains(&x) {
                    panic!("s33 range overflow")
                }
                wr.write_s64(x)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

impl MemArg {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (align, bytes) = bytes.advance_u32()?;
        let (offset, bytes) = bytes.advance_u32()?;
        Ok((Self { align, offset }, bytes))
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        wr.write_u32(self.align)?;
        wr.write_u32(self.offset)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    // Control instructions
    Unreachable,
    Nop,
    Block(BlockType, Vec<Instruction>),
    Loop(BlockType, Vec<Instruction>),
    If(BlockType, Vec<Instruction>, Option<Vec<Instruction>>),
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>, u32),
    Return,
    Call(u32),
    CallIndirect { ty: u32, table: u32 },

    // Reference instructions
    RefNull(ReferenceType),
    RefIsNull,
    RefFunc(u32),

    // Parametric instructions
    Drop,
    SelectNumeric,
    Select(Vec<ValueType>),

    // Variable instructions
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Table instructions
    TableGet(u32),
    TableSet(u32),
    TableInit(u32, u32),
    ElemDrop(u32),
    TableCopy(u32, u32),
    TableGrow(u32),
    TableSize(u32),
    TableFill(u32),

    // Memory instructions
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,
    MemoryInit(u32),
    DataDrop(u32),
    MemoryCopy,
    MemoryFill,

    // Numeric instructions
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),

    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,

    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,

    // Vector instructions
    V128Load(MemArg),
    V128Load8x8S(MemArg),
    V128Load8x8U(MemArg),
    V128Load16x4S(MemArg),
    V128Load16x4U(MemArg),
    V128Load32x2S(MemArg),
    V128Load32x2U(MemArg),
    V128Load8Splat(MemArg),
    V128Load16Splat(MemArg),
    V128Load32Splat(MemArg),
    V128Load64Splat(MemArg),
    V128Load32Zero(MemArg),
    V128Load64Zero(MemArg),
    V128Store(MemArg),
    V128Load8Lane(MemArg, u32),
    V128Load16Lane(MemArg, u32),
    V128Load32Lane(MemArg, u32),
    V128Load64Lane(MemArg, u32),
    V128Store8Lane(MemArg, u32),
    V128Store16Lane(MemArg, u32),
    V128Store32Lane(MemArg, u32),
    V128Store64Lane(MemArg, u32),

    V128Const(u128),

    I8x16Shuffle([u32; 16]),

    I8x16ExtractLaneS(u32),
    I8x16ExtractLaneU(u32),
    I8x16ReplaceLane(u32),
    I16x8ExtractLaneS(u32),
    I16x8ExtractLaneU(u32),
    I16x8ReplaceLane(u32),
    I32x4ExtractLane(u32),
    I32x4ReplaceLane(u32),
    I64x2ExtractLane(u32),
    I64x2ReplaceLane(u32),
    F32x4ExtractLane(u32),
    F32x4ReplaceLane(u32),
    F64x2ExtractLane(u32),
    F64x2ReplaceLane(u32),

    I8X16Swizzle,
    I8x16Splat,
    I16x8Splat,
    I32x4Splat,
    I64x2Splat,
    F32x4Splat,
    F64x2Splat,

    I8x16Eq,
    I8x16Ne,
    I8X16LtS,
    I8X16LtU,
    I8X16GtS,
    I8X16GtU,
    I8X16LeS,
    I8X16LeU,
    I8X16GeS,
    I8X16GeU,

    I16x8Eq,
    I16x8Ne,
    I16x8LtS,
    I16x8LtU,
    I16x8GtS,
    I16x8GtU,
    I16x8LeS,
    I16x8LeU,
    I16x8GeS,
    I16x8GeU,

    I32x4Eq,
    I32x4Ne,
    I32x4LtS,
    I32x4LtU,
    I32x4GtS,
    I32x4GtU,
    I32x4LeS,
    I32x4LeU,
    I32x4GeS,
    I32x4GeU,

    I64x2Eq,
    I64x2Ne,
    I64x2LtS,
    I64x2GtS,
    I64x2LeS,
    I64x2GeS,

    F32x4Eq,
    F32x4Ne,
    F32x4Lt,
    F32x4Gt,
    F32x4Le,
    F32x4Ge,

    F64x2Eq,
    F64x2Ne,
    F64x2Lt,
    F64x2Gt,
    F64x2Le,
    F64x2Ge,

    V128Not,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128Bitselect,
    V128AnyTrue,

    I8x16Abs,
    I8x16Neg,
    I8x16Popcnt,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    I8x16Shl,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSatS,
    I8x16AddSatU,
    I8x16Sub,
    I8x16SubSatS,
    I8x16SubSatU,
    I8x16MinS,
    I8x16MinU,
    I8x16MaxS,
    I8x16MaxU,
    I8x16AvgrU,

    I16x8ExtAddPairwiseI8x16S,
    I16x8ExtAddPairwiseI8x16U,
    I16x8Abs,
    I16x8Neg,
    I16x8Q15MulrSatS,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8ExtendLowI8X16S,
    I16x8ExtendHighI8X16S,
    I16x8ExtendLowI8X16U,
    I16x8ExtendHighI8X16U,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSatS,
    I16x8AddSatU,
    I16x8Sub,
    I16x8SubSatS,
    I16x8SubSatU,
    I16X8Mul,
    I16x8MinS,
    I16x8MinU,
    I16x8MaxS,
    I16x8MaxU,
    I16x8AvgrU,
    I16x8ExtmulLowI8x16S,
    I16x8ExtmulHighI8x16S,
    I16x8ExtmulLowI8x16U,
    I16x8ExtmulHighI8x16U,

    I32x4ExtAddPairwiseI16x8S,
    I32x4ExtAddPairwiseI16x8U,
    I32x4Abs,
    I32x4Neg,
    I32x4AllTrue,
    I32x4Bitmask,
    I32x4ExtendLowI16X8S,
    I32x4ExtendHighI16X8S,
    I32x4ExtendLowI16X8U,
    I32x4ExtendHighI16X8U,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4Sub,
    I32x4Mul,
    I32x4MinS,
    I32x4MinU,
    I32x4MaxS,
    I32x4MaxU,
    I32x4DotI16x8S,
    I32x4ExtmulLowI16x8S,
    I32x4ExtmulHighI16x8S,
    I32x4ExtmulLowI16x8U,
    I32x4ExtmulHighI16x8U,

    I64x2Abs,
    I64x2Neg,
    I64x2AllTrue,
    I64x2Bitmask,
    I64x2ExtendLowI16X8S,
    I64x2ExtendHighI16X8S,
    I64x2ExtendLowI16X8U,
    I64x2ExtendHighI16X8U,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub,
    I64x2Mul,
    I64x2ExtmulLowI32x4S,
    I64x2ExtmulHighI32x4S,
    I64x2ExtmulLowI32x4U,
    I64x2ExtmulHighI32x4U,

    F32x4Ceil,
    F32x4Floor,
    F32x4Trunc,
    F32x4Nearest,
    F32x4Abs,
    F32x4Neg,
    F32x4Sqrt,
    F32x4Add,
    F32x4Sub,
    F32x4Mul,
    F32x4Div,
    F32x4Min,
    F32x4Max,
    F32x4Pmin,
    F32x4Pmax,

    F64x2Ceil,
    F64x2Floor,
    F64x2Trunc,
    F64x2Nearest,
    F64x2Abs,
    F64x2Neg,
    F64x2Sqrt,
    F64x2Add,
    F64x2Sub,
    F64x2Mul,
    F64x2Div,
    F64x2Min,
    F64x2Max,
    F64x2Pmin,
    F64x2Pmax,

    I32x4TruncSatF32x4S,
    I32x4TruncSatF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I32x4TruncSatF64x2SZero,
    I32x4TruncSatF64x2UZero,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,
    F32x4DemoteF64x2Zero,
    F64x2PromoteLowF32x4,
}

impl Instruction {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (&[opcode], bytes) = bytes.advance()?;
        log::trace!("opcode: 0x{opcode:02x}");
        match opcode {
            0x00 => Ok((Self::Unreachable, bytes)),
            0x01 => Ok((Self::Nop, bytes)),
            0x02 => {
                let (bt, bytes) = BlockType::from_bytes(bytes)?;
                let (instrs, _, bytes) = Self::from_bytes_vec(bytes, &[0x0B])?;
                Ok((Self::Block(bt, instrs), bytes))
            }
            0x03 => {
                let (bt, bytes) = BlockType::from_bytes(bytes)?;
                let (instrs, _, bytes) = Self::from_bytes_vec(bytes, &[0x0B])?;
                Ok((Self::Loop(bt, instrs), bytes))
            }
            0x04 => {
                let (bt, bytes) = BlockType::from_bytes(bytes)?;
                let (instrs, end, bytes) = Self::from_bytes_vec(bytes, &[0x05, 0x0B])?;
                match end {
                    0x05 => {
                        let (elseinstrs, _, bytes) = Self::from_bytes_vec(bytes, &[0x0B])?;
                        Ok((Self::If(bt, instrs, Some(elseinstrs)), bytes))
                    }
                    0x0B => Ok((Self::If(bt, instrs, None), bytes)),
                    _ => unreachable!(),
                }
            }
            0x0C => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::Br(li), bytes))
            }
            0x0D => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::BrIf(li), bytes))
            }
            0x0E => {
                let mut liit = bytes.advance_vector(<&[u8]>::advance_u32)?;
                let mut lis = Vec::new();
                for li in &mut liit {
                    lis.push(li?);
                }
                let bytes = liit.finalize();
                let (ln, bytes) = bytes.advance_u32()?;
                Ok((Self::BrTable(lis, ln), bytes))
            }
            0x0F => Ok((Self::Return, bytes)),
            0x10 => {
                let (fi, bytes) = bytes.advance_u32()?;
                Ok((Self::Call(fi), bytes))
            }
            0x11 => {
                let (ty, bytes) = bytes.advance_u32()?;
                let (table, bytes) = bytes.advance_u32()?;
                Ok((Self::CallIndirect { ty, table }, bytes))
            }
            0xD0 => {
                let (&[rt], bytes) = bytes.advance()?;
                let rt = ReferenceType::from_byte(rt)?;
                Ok((Self::RefNull(rt), bytes))
            }
            0xD1 => Ok((Self::RefIsNull, bytes)),
            0xD2 => {
                let (fi, bytes) = bytes.advance_u32()?;
                Ok((Self::RefFunc(fi), bytes))
            }
            0x1A => Ok((Self::Drop, bytes)),
            0x1B => Ok((Self::SelectNumeric, bytes)),
            0x1C => {
                let mut vtit = bytes.advance_vector(|x| {
                    let (&[x], bytes) = x.advance()?;
                    Ok((ValueType::from_byte(x)?, bytes))
                })?;
                let mut vts = Vec::new();
                for vt in &mut vtit {
                    vts.push(vt?);
                }

                Ok((Self::Select(vts), vtit.finalize()))
            }
            0x20 => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::LocalGet(li), bytes))
            }
            0x21 => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::LocalSet(li), bytes))
            }
            0x22 => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::LocalTee(li), bytes))
            }
            0x23 => {
                let (gi, bytes) = bytes.advance_u32()?;
                Ok((Self::GlobalGet(gi), bytes))
            }
            0x24 => {
                let (gi, bytes) = bytes.advance_u32()?;
                Ok((Self::GlobalSet(gi), bytes))
            }
            0x25 => {
                let (ti, bytes) = bytes.advance_u32()?;
                Ok((Self::GlobalSet(ti), bytes))
            }
            0x26 => {
                let (ti, bytes) = bytes.advance_u32()?;
                Ok((Self::GlobalSet(ti), bytes))
            }
            0x28 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Load(ma), bytes))
            }
            0x29 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load(ma), bytes))
            }
            0x2A => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::F32Load(ma), bytes))
            }
            0x2B => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::F64Load(ma), bytes))
            }
            0x2C => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Load8S(ma), bytes))
            }
            0x2D => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Load8U(ma), bytes))
            }
            0x2E => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Load16S(ma), bytes))
            }
            0x2F => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Load16U(ma), bytes))
            }
            0x30 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load8S(ma), bytes))
            }
            0x31 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load8U(ma), bytes))
            }
            0x32 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load16S(ma), bytes))
            }
            0x33 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load16U(ma), bytes))
            }
            0x34 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load32S(ma), bytes))
            }
            0x35 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Load32U(ma), bytes))
            }
            0x36 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Store(ma), bytes))
            }
            0x37 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store(ma), bytes))
            }
            0x38 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::F32Store(ma), bytes))
            }
            0x39 => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::F64Store(ma), bytes))
            }
            0x3A => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Store8(ma), bytes))
            }
            0x3B => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Store16(ma), bytes))
            }
            0x3C => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store8(ma), bytes))
            }
            0x3D => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store16(ma), bytes))
            }
            0x3E => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store32(ma), bytes))
            }
            0x3F => {
                let (&[zero], bytes) = bytes.advance()?;
                if zero != 0x00 {
                    return Err(Error::MemoryInstruction(0x3F, zero));
                }
                Ok((Self::MemorySize, bytes))
            }
            0x40 => {
                let (&[zero], bytes) = bytes.advance()?;
                if zero != 0x00 {
                    return Err(Error::MemoryInstruction(0x40, zero));
                }
                Ok((Self::MemoryGrow, bytes))
            }
            0x41 => {
                let (n, bytes) = bytes.advance_u32()?;
                Ok((Self::I32Const(n), bytes))
            }
            0x42 => {
                let (n, bytes) = bytes.advance_u64()?;
                Ok((Self::I64Const(n), bytes))
            }
            0x43 => {
                let (n, bytes) = bytes.advance_f32()?;
                Ok((Self::F32Const(n), bytes))
            }
            0x44 => {
                let (n, bytes) = bytes.advance_f64()?;
                Ok((Self::F64Const(n), bytes))
            }
            0x45 => Ok((Self::I32Eqz, bytes)),
            0x46 => Ok((Self::I32Eq, bytes)),
            0x47 => Ok((Self::I32Ne, bytes)),
            0x48 => Ok((Self::I32LtS, bytes)),
            0x49 => Ok((Self::I32LtU, bytes)),
            0x4A => Ok((Self::I32GtS, bytes)),
            0x4B => Ok((Self::I32GtU, bytes)),
            0x4C => Ok((Self::I32LeS, bytes)),
            0x4D => Ok((Self::I32LeU, bytes)),
            0x4E => Ok((Self::I32GeS, bytes)),
            0x4F => Ok((Self::I32GeU, bytes)),
            0x50 => Ok((Self::I64Eqz, bytes)),
            0x51 => Ok((Self::I64Eq, bytes)),
            0x52 => Ok((Self::I64Ne, bytes)),
            0x53 => Ok((Self::I64LtS, bytes)),
            0x54 => Ok((Self::I64LtU, bytes)),
            0x55 => Ok((Self::I64GtS, bytes)),
            0x56 => Ok((Self::I64GtU, bytes)),
            0x57 => Ok((Self::I64LeS, bytes)),
            0x58 => Ok((Self::I64LeU, bytes)),
            0x59 => Ok((Self::I64GeS, bytes)),
            0x5A => Ok((Self::I64GeU, bytes)),
            0x5B => Ok((Self::F32Eq, bytes)),
            0x5C => Ok((Self::F32Ne, bytes)),
            0x5D => Ok((Self::F32Lt, bytes)),
            0x5E => Ok((Self::F32Gt, bytes)),
            0x5F => Ok((Self::F32Le, bytes)),
            0x60 => Ok((Self::F32Ge, bytes)),
            0x61 => Ok((Self::F64Eq, bytes)),
            0x62 => Ok((Self::F64Ne, bytes)),
            0x63 => Ok((Self::F64Lt, bytes)),
            0x64 => Ok((Self::F64Gt, bytes)),
            0x65 => Ok((Self::F64Le, bytes)),
            0x66 => Ok((Self::F64Ge, bytes)),
            0x67 => Ok((Self::I32Clz, bytes)),
            0x68 => Ok((Self::I32Ctz, bytes)),
            0x69 => Ok((Self::I32Popcnt, bytes)),
            0x6A => Ok((Self::I32Add, bytes)),
            0x6B => Ok((Self::I32Sub, bytes)),
            0x6C => Ok((Self::I32Mul, bytes)),
            0x6D => Ok((Self::I32DivS, bytes)),
            0x6E => Ok((Self::I32DivU, bytes)),
            0x6F => Ok((Self::I32RemS, bytes)),
            0x70 => Ok((Self::I32RemU, bytes)),
            0x71 => Ok((Self::I32And, bytes)),
            0x72 => Ok((Self::I32Or, bytes)),
            0x73 => Ok((Self::I32Xor, bytes)),
            0x74 => Ok((Self::I32Shl, bytes)),
            0x75 => Ok((Self::I32ShrS, bytes)),
            0x76 => Ok((Self::I32ShrU, bytes)),
            0x77 => Ok((Self::I32Rotl, bytes)),
            0x78 => Ok((Self::I32Rotr, bytes)),
            0x79 => Ok((Self::I64Clz, bytes)),
            0x7A => Ok((Self::I64Ctz, bytes)),
            0x7B => Ok((Self::I64Popcnt, bytes)),
            0x7C => Ok((Self::I64Add, bytes)),
            0x7D => Ok((Self::I64Sub, bytes)),
            0x7E => Ok((Self::I64Mul, bytes)),
            0x7F => Ok((Self::I64DivS, bytes)),
            0x80 => Ok((Self::I64DivU, bytes)),
            0x81 => Ok((Self::I64RemS, bytes)),
            0x82 => Ok((Self::I64RemU, bytes)),
            0x83 => Ok((Self::I64And, bytes)),
            0x84 => Ok((Self::I64Or, bytes)),
            0x85 => Ok((Self::I64Xor, bytes)),
            0x86 => Ok((Self::I64Shl, bytes)),
            0x87 => Ok((Self::I64ShrS, bytes)),
            0x88 => Ok((Self::I64ShrU, bytes)),
            0x89 => Ok((Self::I64Rotl, bytes)),
            0x8A => Ok((Self::I64Rotr, bytes)),
            0x8B => Ok((Self::F32Abs, bytes)),
            0x8C => Ok((Self::F32Neg, bytes)),
            0x8D => Ok((Self::F32Ceil, bytes)),
            0x8E => Ok((Self::F32Floor, bytes)),
            0x8F => Ok((Self::F32Trunc, bytes)),
            0x90 => Ok((Self::F32Nearest, bytes)),
            0x91 => Ok((Self::F32Sqrt, bytes)),
            0x92 => Ok((Self::F32Add, bytes)),
            0x93 => Ok((Self::F32Sub, bytes)),
            0x94 => Ok((Self::F32Mul, bytes)),
            0x95 => Ok((Self::F32Div, bytes)),
            0x96 => Ok((Self::F32Min, bytes)),
            0x97 => Ok((Self::F32Max, bytes)),
            0x98 => Ok((Self::F32Copysign, bytes)),
            0x99 => Ok((Self::F64Abs, bytes)),
            0x9A => Ok((Self::F64Neg, bytes)),
            0x9B => Ok((Self::F64Ceil, bytes)),
            0x9C => Ok((Self::F64Floor, bytes)),
            0x9D => Ok((Self::F64Trunc, bytes)),
            0x9E => Ok((Self::F64Nearest, bytes)),
            0x9F => Ok((Self::F64Sqrt, bytes)),
            0xA0 => Ok((Self::F64Add, bytes)),
            0xA1 => Ok((Self::F64Sub, bytes)),
            0xA2 => Ok((Self::F64Mul, bytes)),
            0xA3 => Ok((Self::F64Div, bytes)),
            0xA4 => Ok((Self::F64Min, bytes)),
            0xA5 => Ok((Self::F64Max, bytes)),
            0xA6 => Ok((Self::F64Copysign, bytes)),
            0xA7 => Ok((Self::I32WrapI64, bytes)),
            0xA8 => Ok((Self::I32TruncF32S, bytes)),
            0xA9 => Ok((Self::I32TruncF32U, bytes)),
            0xAA => Ok((Self::I32TruncF64S, bytes)),
            0xAB => Ok((Self::I32TruncF64U, bytes)),
            0xAC => Ok((Self::I64ExtendI32S, bytes)),
            0xAD => Ok((Self::I64ExtendI32U, bytes)),
            0xAE => Ok((Self::I64TruncF32S, bytes)),
            0xAF => Ok((Self::I64TruncF32U, bytes)),
            0xB0 => Ok((Self::I64TruncF64S, bytes)),
            0xB1 => Ok((Self::I64TruncF64U, bytes)),
            0xB2 => Ok((Self::F32ConvertI32S, bytes)),
            0xB3 => Ok((Self::F32ConvertI32U, bytes)),
            0xB4 => Ok((Self::F32ConvertI64S, bytes)),
            0xB5 => Ok((Self::F32ConvertI64U, bytes)),
            0xB6 => Ok((Self::F32DemoteF64, bytes)),
            0xB7 => Ok((Self::F64ConvertI32S, bytes)),
            0xB8 => Ok((Self::F64ConvertI32U, bytes)),
            0xB9 => Ok((Self::F64ConvertI64S, bytes)),
            0xBA => Ok((Self::F64ConvertI64U, bytes)),
            0xBB => Ok((Self::F64PromoteF32, bytes)),
            0xBC => Ok((Self::I32ReinterpretF32, bytes)),
            0xBD => Ok((Self::I64ReinterpretF64, bytes)),
            0xBE => Ok((Self::F32ReinterpretI32, bytes)),
            0xBF => Ok((Self::F64ReinterpretI64, bytes)),
            0xC0 => Ok((Self::I32Extend8S, bytes)),
            0xC1 => Ok((Self::I32Extend16S, bytes)),
            0xC2 => Ok((Self::I64Extend8S, bytes)),
            0xC3 => Ok((Self::I64Extend16S, bytes)),
            0xC4 => Ok((Self::I64Extend32S, bytes)),
            0xFC => {
                let (subop, bytes) = bytes.advance_u32()?;
                match subop {
                    0 => Ok((Self::I32TruncSatF32S, bytes)),
                    1 => Ok((Self::I32TruncSatF32U, bytes)),
                    2 => Ok((Self::I32TruncSatF64S, bytes)),
                    3 => Ok((Self::I32TruncSatF64U, bytes)),
                    4 => Ok((Self::I64TruncSatF32S, bytes)),
                    5 => Ok((Self::I64TruncSatF32U, bytes)),
                    6 => Ok((Self::I64TruncSatF64S, bytes)),
                    7 => Ok((Self::I64TruncSatF64U, bytes)),
                    8 => {
                        let (di, bytes) = bytes.advance_u32()?;
                        let (&[zero], bytes) = bytes.advance()?;
                        if zero != 0x00 {
                            return Err(Error::MemoryInstructionNoTrailingZero {
                                instr: "memory.init",
                                byte: zero,
                            });
                        }
                        Ok((Self::MemoryInit(di), bytes))
                    }
                    9 => {
                        let (di, bytes) = bytes.advance_u32()?;
                        Ok((Self::DataDrop(di), bytes))
                    }
                    10 => {
                        let (&[zero], bytes) = bytes.advance()?;
                        if zero != 0x00 {
                            return Err(Error::MemoryInstructionNoTrailingZero {
                                instr: "memory.copy(first)",
                                byte: zero,
                            });
                        }
                        let (&[zero], bytes) = bytes.advance()?;
                        if zero != 0x00 {
                            return Err(Error::MemoryInstructionNoTrailingZero {
                                instr: "memory.copy(second)",
                                byte: zero,
                            });
                        }
                        Ok((Self::MemoryCopy, bytes))
                    }
                    11 => {
                        let (&[zero], bytes) = bytes.advance()?;
                        if zero != 0x00 {
                            return Err(Error::MemoryInstructionNoTrailingZero {
                                instr: "memory.fill",
                                byte: zero,
                            });
                        }
                        Ok((Self::MemoryFill, bytes))
                    }
                    12 => {
                        let (ei, bytes) = bytes.advance_u32()?;
                        let (ti, bytes) = bytes.advance_u32()?;
                        Ok((Self::TableInit(ei, ti), bytes))
                    }
                    13 => {
                        let (ei, bytes) = bytes.advance_u32()?;
                        Ok((Self::ElemDrop(ei), bytes))
                    }
                    14 => {
                        let (ti1, bytes) = bytes.advance_u32()?;
                        let (ti2, bytes) = bytes.advance_u32()?;
                        Ok((Self::TableCopy(ti1, ti2), bytes))
                    }
                    15 => {
                        let (ti, bytes) = bytes.advance_u32()?;
                        Ok((Self::TableGrow(ti), bytes))
                    }
                    16 => {
                        let (ti, bytes) = bytes.advance_u32()?;
                        Ok((Self::TableSize(ti), bytes))
                    }
                    17 => {
                        let (ti, bytes) = bytes.advance_u32()?;
                        Ok((Self::TableFill(ti), bytes))
                    }
                    _ => Err(Error::HexFcInstructionSubopcode(subop)),
                }
            }
            0xFD => {
                let (subop, bytes) = bytes.advance_u32()?;
                match subop {
                    0 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load(ma), bytes))
                    }
                    1 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load8x8S(ma), bytes))
                    }
                    2 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load8x8U(ma), bytes))
                    }
                    3 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load16x4S(ma), bytes))
                    }
                    4 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load16x4U(ma), bytes))
                    }
                    5 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load32x2S(ma), bytes))
                    }
                    6 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load32x2U(ma), bytes))
                    }
                    7 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load8Splat(ma), bytes))
                    }
                    8 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load16Splat(ma), bytes))
                    }
                    9 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load32Splat(ma), bytes))
                    }
                    10 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load64Splat(ma), bytes))
                    }
                    92 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load32Zero(ma), bytes))
                    }
                    93 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Load64Zero(ma), bytes))
                    }
                    11 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        Ok((Self::V128Store(ma), bytes))
                    }
                    84 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Load8Lane(ma, li), bytes))
                    }
                    85 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Load16Lane(ma, li), bytes))
                    }
                    86 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Load32Lane(ma, li), bytes))
                    }
                    87 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Load64Lane(ma, li), bytes))
                    }
                    88 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Store8Lane(ma, li), bytes))
                    }
                    89 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Store16Lane(ma, li), bytes))
                    }
                    90 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Store32Lane(ma, li), bytes))
                    }
                    91 => {
                        let (ma, bytes) = MemArg::from_bytes(bytes)?;
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::V128Store64Lane(ma, li), bytes))
                    }
                    12 => {
                        let (v128, bytes) = bytes.advance::<16>()?;
                        Ok((Self::V128Const(u128::from_le_bytes(*v128)), bytes))
                    }
                    13 => {
                        let mut lis = [0u32; 16];
                        let mut bytes = bytes;
                        for li in &mut lis {
                            let (li_, bytes_) = bytes.advance_u32()?;
                            *li = li_;
                            bytes = bytes_;
                        }
                        Ok((Self::I8x16Shuffle(lis), bytes))
                    }
                    21 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I8x16ExtractLaneS(li), bytes))
                    }
                    22 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I8x16ExtractLaneU(li), bytes))
                    }
                    23 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I8x16ReplaceLane(li), bytes))
                    }
                    24 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I16x8ExtractLaneS(li), bytes))
                    }
                    25 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I16x8ExtractLaneU(li), bytes))
                    }
                    26 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I16x8ReplaceLane(li), bytes))
                    }
                    27 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I32x4ExtractLane(li), bytes))
                    }
                    28 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I32x4ReplaceLane(li), bytes))
                    }
                    29 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I64x2ExtractLane(li), bytes))
                    }
                    30 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::I64x2ReplaceLane(li), bytes))
                    }
                    31 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::F32x4ExtractLane(li), bytes))
                    }
                    32 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::F32x4ReplaceLane(li), bytes))
                    }
                    33 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::F64x2ExtractLane(li), bytes))
                    }
                    34 => {
                        let (li, bytes) = bytes.advance_u32()?;
                        Ok((Self::F64x2ReplaceLane(li), bytes))
                    }
                    14 => Ok((Self::I8X16Swizzle, bytes)),
                    15 => Ok((Self::I8x16Splat, bytes)),
                    16 => Ok((Self::I16x8Splat, bytes)),
                    17 => Ok((Self::I32x4Splat, bytes)),
                    18 => Ok((Self::I64x2Splat, bytes)),
                    19 => Ok((Self::F32x4Splat, bytes)),
                    20 => Ok((Self::F64x2Splat, bytes)),
                    35 => Ok((Self::I8x16Eq, bytes)),
                    36 => Ok((Self::I8x16Ne, bytes)),
                    37 => Ok((Self::I8X16LtS, bytes)),
                    38 => Ok((Self::I8X16LtU, bytes)),
                    39 => Ok((Self::I8X16GtS, bytes)),
                    40 => Ok((Self::I8X16GtU, bytes)),
                    41 => Ok((Self::I8X16LeS, bytes)),
                    42 => Ok((Self::I8X16LeU, bytes)),
                    43 => Ok((Self::I8X16GeS, bytes)),
                    44 => Ok((Self::I8X16GeU, bytes)),
                    45 => Ok((Self::I16x8Eq, bytes)),
                    46 => Ok((Self::I16x8Ne, bytes)),
                    47 => Ok((Self::I16x8LtS, bytes)),
                    48 => Ok((Self::I16x8LtU, bytes)),
                    49 => Ok((Self::I16x8GtS, bytes)),
                    50 => Ok((Self::I16x8GtU, bytes)),
                    51 => Ok((Self::I16x8LeS, bytes)),
                    52 => Ok((Self::I16x8LeU, bytes)),
                    53 => Ok((Self::I16x8GeS, bytes)),
                    54 => Ok((Self::I16x8GeU, bytes)),
                    55 => Ok((Self::I32x4Eq, bytes)),
                    56 => Ok((Self::I32x4Ne, bytes)),
                    57 => Ok((Self::I32x4LtS, bytes)),
                    58 => Ok((Self::I32x4LtU, bytes)),
                    59 => Ok((Self::I32x4GtS, bytes)),
                    60 => Ok((Self::I32x4GtU, bytes)),
                    61 => Ok((Self::I32x4LeS, bytes)),
                    62 => Ok((Self::I32x4LeU, bytes)),
                    63 => Ok((Self::I32x4GeS, bytes)),
                    64 => Ok((Self::I32x4GeU, bytes)),
                    214 => Ok((Self::I64x2Eq, bytes)),
                    215 => Ok((Self::I64x2Ne, bytes)),
                    216 => Ok((Self::I64x2LtS, bytes)),
                    217 => Ok((Self::I64x2GtS, bytes)),
                    218 => Ok((Self::I64x2LeS, bytes)),
                    219 => Ok((Self::I64x2GeS, bytes)),
                    65 => Ok((Self::F32x4Eq, bytes)),
                    66 => Ok((Self::F32x4Ne, bytes)),
                    67 => Ok((Self::F32x4Lt, bytes)),
                    68 => Ok((Self::F32x4Gt, bytes)),
                    69 => Ok((Self::F32x4Le, bytes)),
                    70 => Ok((Self::F32x4Ge, bytes)),
                    71 => Ok((Self::F64x2Eq, bytes)),
                    72 => Ok((Self::F64x2Ne, bytes)),
                    73 => Ok((Self::F64x2Lt, bytes)),
                    74 => Ok((Self::F64x2Gt, bytes)),
                    75 => Ok((Self::F64x2Le, bytes)),
                    76 => Ok((Self::F64x2Ge, bytes)),
                    77 => Ok((Self::V128Not, bytes)),
                    78 => Ok((Self::V128And, bytes)),
                    79 => Ok((Self::V128AndNot, bytes)),
                    80 => Ok((Self::V128Or, bytes)),
                    81 => Ok((Self::V128Xor, bytes)),
                    82 => Ok((Self::V128Bitselect, bytes)),
                    83 => Ok((Self::V128AnyTrue, bytes)),
                    96 => Ok((Self::I8x16Abs, bytes)),
                    97 => Ok((Self::I8x16Neg, bytes)),
                    98 => Ok((Self::I8x16Popcnt, bytes)),
                    99 => Ok((Self::I8x16AllTrue, bytes)),
                    100 => Ok((Self::I8x16Bitmask, bytes)),
                    101 => Ok((Self::I8x16NarrowI16x8S, bytes)),
                    102 => Ok((Self::I8x16NarrowI16x8U, bytes)),
                    107 => Ok((Self::I8x16Shl, bytes)),
                    108 => Ok((Self::I8x16ShrS, bytes)),
                    109 => Ok((Self::I8x16ShrU, bytes)),
                    110 => Ok((Self::I8x16Add, bytes)),
                    111 => Ok((Self::I8x16AddSatS, bytes)),
                    112 => Ok((Self::I8x16AddSatU, bytes)),
                    113 => Ok((Self::I8x16Sub, bytes)),
                    114 => Ok((Self::I8x16SubSatS, bytes)),
                    115 => Ok((Self::I8x16SubSatU, bytes)),
                    118 => Ok((Self::I8x16MinS, bytes)),
                    119 => Ok((Self::I8x16MinU, bytes)),
                    120 => Ok((Self::I8x16MaxS, bytes)),
                    121 => Ok((Self::I8x16MaxU, bytes)),
                    123 => Ok((Self::I8x16AvgrU, bytes)),
                    124 => Ok((Self::I16x8ExtAddPairwiseI8x16S, bytes)),
                    125 => Ok((Self::I16x8ExtAddPairwiseI8x16U, bytes)),
                    128 => Ok((Self::I16x8Abs, bytes)),
                    129 => Ok((Self::I16x8Neg, bytes)),
                    130 => Ok((Self::I16x8Q15MulrSatS, bytes)),
                    131 => Ok((Self::I16x8AllTrue, bytes)),
                    132 => Ok((Self::I16x8Bitmask, bytes)),
                    133 => Ok((Self::I16x8NarrowI32x4S, bytes)),
                    134 => Ok((Self::I16x8NarrowI32x4U, bytes)),
                    135 => Ok((Self::I16x8ExtendLowI8X16S, bytes)),
                    136 => Ok((Self::I16x8ExtendHighI8X16S, bytes)),
                    137 => Ok((Self::I16x8ExtendLowI8X16U, bytes)),
                    138 => Ok((Self::I16x8ExtendHighI8X16U, bytes)),
                    139 => Ok((Self::I16x8Shl, bytes)),
                    140 => Ok((Self::I16x8ShrS, bytes)),
                    141 => Ok((Self::I16x8ShrU, bytes)),
                    142 => Ok((Self::I16x8Add, bytes)),
                    143 => Ok((Self::I16x8AddSatS, bytes)),
                    144 => Ok((Self::I16x8AddSatU, bytes)),
                    145 => Ok((Self::I16x8Sub, bytes)),
                    146 => Ok((Self::I16x8SubSatS, bytes)),
                    147 => Ok((Self::I16x8SubSatU, bytes)),
                    149 => Ok((Self::I16X8Mul, bytes)),
                    150 => Ok((Self::I16x8MinS, bytes)),
                    151 => Ok((Self::I16x8MinU, bytes)),
                    152 => Ok((Self::I16x8MaxS, bytes)),
                    153 => Ok((Self::I16x8MaxU, bytes)),
                    155 => Ok((Self::I16x8AvgrU, bytes)),
                    156 => Ok((Self::I16x8ExtmulLowI8x16S, bytes)),
                    157 => Ok((Self::I16x8ExtmulHighI8x16S, bytes)),
                    158 => Ok((Self::I16x8ExtmulLowI8x16U, bytes)),
                    159 => Ok((Self::I16x8ExtmulHighI8x16U, bytes)),
                    126 => Ok((Self::I32x4ExtAddPairwiseI16x8S, bytes)),
                    127 => Ok((Self::I32x4ExtAddPairwiseI16x8U, bytes)),
                    160 => Ok((Self::I32x4Abs, bytes)),
                    161 => Ok((Self::I32x4Neg, bytes)),
                    163 => Ok((Self::I32x4AllTrue, bytes)),
                    164 => Ok((Self::I32x4Bitmask, bytes)),
                    167 => Ok((Self::I32x4ExtendLowI16X8S, bytes)),
                    168 => Ok((Self::I32x4ExtendHighI16X8S, bytes)),
                    169 => Ok((Self::I32x4ExtendLowI16X8U, bytes)),
                    170 => Ok((Self::I32x4ExtendHighI16X8U, bytes)),
                    171 => Ok((Self::I32x4Shl, bytes)),
                    172 => Ok((Self::I32x4ShrS, bytes)),
                    173 => Ok((Self::I32x4ShrU, bytes)),
                    174 => Ok((Self::I32x4Add, bytes)),
                    177 => Ok((Self::I32x4Sub, bytes)),
                    181 => Ok((Self::I32x4Mul, bytes)),
                    182 => Ok((Self::I32x4MinS, bytes)),
                    183 => Ok((Self::I32x4MinU, bytes)),
                    184 => Ok((Self::I32x4MaxS, bytes)),
                    185 => Ok((Self::I32x4MaxU, bytes)),
                    186 => Ok((Self::I32x4DotI16x8S, bytes)),
                    188 => Ok((Self::I32x4ExtmulLowI16x8S, bytes)),
                    189 => Ok((Self::I32x4ExtmulHighI16x8S, bytes)),
                    190 => Ok((Self::I32x4ExtmulLowI16x8U, bytes)),
                    191 => Ok((Self::I32x4ExtmulHighI16x8U, bytes)),
                    192 => Ok((Self::I64x2Abs, bytes)),
                    193 => Ok((Self::I64x2Neg, bytes)),
                    195 => Ok((Self::I64x2AllTrue, bytes)),
                    196 => Ok((Self::I64x2Bitmask, bytes)),
                    199 => Ok((Self::I64x2ExtendLowI16X8S, bytes)),
                    200 => Ok((Self::I64x2ExtendHighI16X8S, bytes)),
                    201 => Ok((Self::I64x2ExtendLowI16X8U, bytes)),
                    202 => Ok((Self::I64x2ExtendHighI16X8U, bytes)),
                    203 => Ok((Self::I64x2Shl, bytes)),
                    204 => Ok((Self::I64x2ShrS, bytes)),
                    205 => Ok((Self::I64x2ShrU, bytes)),
                    206 => Ok((Self::I64x2Add, bytes)),
                    209 => Ok((Self::I64x2Sub, bytes)),
                    213 => Ok((Self::I64x2Mul, bytes)),
                    220 => Ok((Self::I64x2ExtmulLowI32x4S, bytes)),
                    221 => Ok((Self::I64x2ExtmulHighI32x4S, bytes)),
                    222 => Ok((Self::I64x2ExtmulLowI32x4U, bytes)),
                    223 => Ok((Self::I64x2ExtmulHighI32x4U, bytes)),
                    103 => Ok((Self::F32x4Ceil, bytes)),
                    104 => Ok((Self::F32x4Floor, bytes)),
                    105 => Ok((Self::F32x4Trunc, bytes)),
                    106 => Ok((Self::F32x4Nearest, bytes)),
                    224 => Ok((Self::F32x4Abs, bytes)),
                    225 => Ok((Self::F32x4Neg, bytes)),
                    227 => Ok((Self::F32x4Sqrt, bytes)),
                    228 => Ok((Self::F32x4Add, bytes)),
                    229 => Ok((Self::F32x4Sub, bytes)),
                    230 => Ok((Self::F32x4Mul, bytes)),
                    231 => Ok((Self::F32x4Div, bytes)),
                    232 => Ok((Self::F32x4Min, bytes)),
                    233 => Ok((Self::F32x4Max, bytes)),
                    234 => Ok((Self::F32x4Pmin, bytes)),
                    235 => Ok((Self::F32x4Pmax, bytes)),
                    116 => Ok((Self::F64x2Ceil, bytes)),
                    117 => Ok((Self::F64x2Floor, bytes)),
                    122 => Ok((Self::F64x2Trunc, bytes)),
                    148 => Ok((Self::F64x2Nearest, bytes)),
                    236 => Ok((Self::F64x2Abs, bytes)),
                    237 => Ok((Self::F64x2Neg, bytes)),
                    239 => Ok((Self::F64x2Sqrt, bytes)),
                    240 => Ok((Self::F64x2Add, bytes)),
                    241 => Ok((Self::F64x2Sub, bytes)),
                    242 => Ok((Self::F64x2Mul, bytes)),
                    243 => Ok((Self::F64x2Div, bytes)),
                    244 => Ok((Self::F64x2Min, bytes)),
                    245 => Ok((Self::F64x2Max, bytes)),
                    246 => Ok((Self::F64x2Pmin, bytes)),
                    247 => Ok((Self::F64x2Pmax, bytes)),
                    248 => Ok((Self::I32x4TruncSatF32x4S, bytes)),
                    249 => Ok((Self::I32x4TruncSatF32x4U, bytes)),
                    250 => Ok((Self::F32x4ConvertI32x4S, bytes)),
                    251 => Ok((Self::F32x4ConvertI32x4U, bytes)),
                    252 => Ok((Self::I32x4TruncSatF64x2SZero, bytes)),
                    253 => Ok((Self::I32x4TruncSatF64x2UZero, bytes)),
                    254 => Ok((Self::F64x2ConvertLowI32x4S, bytes)),
                    255 => Ok((Self::F64x2ConvertLowI32x4U, bytes)),
                    94 => Ok((Self::F32x4DemoteF64x2Zero, bytes)),
                    95 => Ok((Self::F64x2PromoteLowF32x4, bytes)),
                    _ => Err(Error::VectorInstructionSubopcode(subop)),
                }
            }
            _ => Err(Error::Opcode(opcode)),
        }
    }

    pub(crate) fn from_bytes_vec<'a, 'bytes>(
        mut bytes: &'bytes [u8],
        endset: &'a [u8],
    ) -> Result<(Vec<Self>, u8, &'bytes [u8]), Error> {
        let mut ret = Vec::new();
        loop {
            if bytes.is_empty() {
                return Err(Error::UnexpectedEof(1, 0));
            }
            if endset.contains(&bytes[0]) {
                return Ok((ret, bytes[0], &bytes[1..]));
            }
            let (instr, left) = Self::from_bytes(bytes)?;
            ret.push(instr);
            bytes = left;
        }
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        match self {
            Instruction::Unreachable => {
                wr.write_all(&[0x00])?;
            }
            Instruction::Nop => {
                wr.write_all(&[0x01])?;
            }
            Instruction::Block(bt, instrs) => {
                wr.write_all(&[0x02])?;
                bt.write_into(wr)?;
                Self::write_slice_into(instrs, wr)?;
                wr.write_all(&[0x0B])?;
            }
            Instruction::Loop(_bt, instrs) => {
                wr.write_all(&[0x03])?;
                Self::write_slice_into(instrs, wr)?;
                wr.write_all(&[0x0B])?;
            }
            Instruction::If(_bt, instrs, elseinstrs) => {
                wr.write_all(&[0x04])?;
                Self::write_slice_into(instrs, wr)?;
                if let Some(elseinstrs) = elseinstrs {
                    wr.write_all(&[0x05])?;
                    Self::write_slice_into(elseinstrs, wr)?;
                    wr.write_all(&[0x0B])?;
                } else {
                    wr.write_all(&[0x0B])?;
                }
            }
            Instruction::Br(li) => {
                wr.write_all(&[0x0C])?;
                wr.write_u32(*li)?;
            }
            Instruction::BrIf(li) => {
                wr.write_all(&[0x0D])?;
                wr.write_u32(*li)?;
            }
            Instruction::BrTable(lis, _ln) => {
                wr.write_all(&[0x0E])?;
                wr.write_vector(lis, |x, wr| wr.write_u32(*x))?;
            }
            Instruction::Return => {
                wr.write_all(&[0x0F])?;
            }
            Instruction::Call(fi) => {
                wr.write_all(&[0x10])?;
                wr.write_u32(*fi)?;
            }
            Instruction::CallIndirect { ty, table } => {
                wr.write_all(&[0x11])?;
                wr.write_u32(*ty)?;
                wr.write_u32(*table)?;
            }
            Instruction::RefNull(_) => {
                wr.write_all(&[0xD0])?;
            }
            Instruction::RefIsNull => {
                wr.write_all(&[0xD1])?;
            }
            Instruction::RefFunc(fi) => {
                wr.write_all(&[0xD2])?;
                wr.write_u32(*fi)?;
            }
            Instruction::Drop => {
                wr.write_all(&[0x1A])?;
            }
            Instruction::SelectNumeric => {
                wr.write_all(&[0x1B])?;
            }
            Instruction::Select(vts) => {
                wr.write_all(&[0x1C])?;
                wr.write_vector(vts, ValueType::write_into)?;
            }
            Instruction::LocalGet(li) => {
                wr.write_all(&[0x20])?;
                wr.write_u32(*li)?;
            }
            Instruction::LocalSet(li) => {
                wr.write_all(&[0x21])?;
                wr.write_u32(*li)?;
            }
            Instruction::LocalTee(li) => {
                wr.write_all(&[0x22])?;
                wr.write_u32(*li)?;
            }
            Instruction::GlobalGet(gi) => {
                wr.write_all(&[0x23])?;
                wr.write_u32(*gi)?;
            }
            Instruction::GlobalSet(gi) => {
                wr.write_all(&[0x24])?;
                wr.write_u32(*gi)?;
            }
            Instruction::TableGet(ti) => {
                wr.write_all(&[0x25])?;
                wr.write_u32(*ti)?;
            }
            Instruction::TableSet(ti) => {
                wr.write_all(&[0x26])?;
                wr.write_u32(*ti)?;
            }
            Instruction::TableInit(ei, ti) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(12)?;
                wr.write_u32(*ei)?;
                wr.write_u32(*ti)?;
            }
            Instruction::ElemDrop(ei) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(13)?;
                wr.write_u32(*ei)?;
            }
            Instruction::TableCopy(ti1, ti2) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(14)?;
                wr.write_u32(*ti1)?;
                wr.write_u32(*ti2)?;
            }
            Instruction::TableGrow(ti) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(15)?;
                wr.write_u32(*ti)?;
            }
            Instruction::TableSize(ti) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(16)?;
                wr.write_u32(*ti)?;
            }
            Instruction::TableFill(ti) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(17)?;
                wr.write_u32(*ti)?;
            }
            Instruction::I32Load(ma) => {
                wr.write_all(&[0x28])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load(ma) => {
                wr.write_all(&[0x29])?;
                ma.write_into(wr)?;
            }
            Instruction::F32Load(ma) => {
                wr.write_all(&[0x2A])?;
                ma.write_into(wr)?;
            }
            Instruction::F64Load(ma) => {
                wr.write_all(&[0x2B])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Load8S(ma) => {
                wr.write_all(&[0x2C])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Load8U(ma) => {
                wr.write_all(&[0x2D])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Load16S(ma) => {
                wr.write_all(&[0x2E])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Load16U(ma) => {
                wr.write_all(&[0x2F])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load8S(ma) => {
                wr.write_all(&[0x30])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load8U(ma) => {
                wr.write_all(&[0x31])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load16S(ma) => {
                wr.write_all(&[0x32])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load16U(ma) => {
                wr.write_all(&[0x33])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load32S(ma) => {
                wr.write_all(&[0x34])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Load32U(ma) => {
                wr.write_all(&[0x35])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Store(ma) => {
                wr.write_all(&[0x36])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Store(ma) => {
                wr.write_all(&[0x37])?;
                ma.write_into(wr)?;
            }
            Instruction::F32Store(ma) => {
                wr.write_all(&[0x38])?;
                ma.write_into(wr)?;
            }
            Instruction::F64Store(ma) => {
                wr.write_all(&[0x39])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Store8(ma) => {
                wr.write_all(&[0x3A])?;
                ma.write_into(wr)?;
            }
            Instruction::I32Store16(ma) => {
                wr.write_all(&[0x3B])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Store8(ma) => {
                wr.write_all(&[0x3C])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Store16(ma) => {
                wr.write_all(&[0x3D])?;
                ma.write_into(wr)?;
            }
            Instruction::I64Store32(ma) => {
                wr.write_all(&[0x3E])?;
                ma.write_into(wr)?;
            }
            Instruction::MemorySize => {
                wr.write_all(&[0x3F, 0x00])?;
            }
            Instruction::MemoryGrow => {
                wr.write_all(&[0x40, 0x00])?;
            }
            Instruction::MemoryInit(di) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(8)?;
                wr.write_u32(*di)?;
                wr.write_all(&[0x00])?;
            }
            Instruction::DataDrop(di) => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(9)?;
                wr.write_u32(*di)?;
            }
            Instruction::MemoryCopy => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(10)?;
                wr.write_all(&[0x00, 0x00])?;
            }
            Instruction::MemoryFill => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(11)?;
                wr.write_all(&[0x00])?;
            }
            Instruction::I32Const(n) => {
                wr.write_all(&[0x41])?;
                wr.write_u32(*n)?;
            }
            Instruction::I64Const(n) => {
                wr.write_all(&[0x42])?;
                wr.write_u64(*n)?;
            }
            Instruction::F32Const(z) => {
                wr.write_all(&[0x43])?;
                wr.write_f32(*z)?;
            }
            Instruction::F64Const(z) => {
                wr.write_all(&[0x44])?;
                wr.write_f64(*z)?;
            }
            Instruction::I32Eqz => {
                wr.write_all(&[0x45])?;
            }
            Instruction::I32Eq => {
                wr.write_all(&[0x46])?;
            }
            Instruction::I32Ne => {
                wr.write_all(&[0x47])?;
            }
            Instruction::I32LtS => {
                wr.write_all(&[0x48])?;
            }
            Instruction::I32LtU => {
                wr.write_all(&[0x49])?;
            }
            Instruction::I32GtS => {
                wr.write_all(&[0x4A])?;
            }
            Instruction::I32GtU => {
                wr.write_all(&[0x4B])?;
            }
            Instruction::I32LeS => {
                wr.write_all(&[0x4C])?;
            }
            Instruction::I32LeU => {
                wr.write_all(&[0x4D])?;
            }
            Instruction::I32GeS => {
                wr.write_all(&[0x4E])?;
            }
            Instruction::I32GeU => {
                wr.write_all(&[0x4F])?;
            }
            Instruction::I64Eqz => {
                wr.write_all(&[0x50])?;
            }
            Instruction::I64Eq => {
                wr.write_all(&[0x51])?;
            }
            Instruction::I64Ne => {
                wr.write_all(&[0x52])?;
            }
            Instruction::I64LtS => {
                wr.write_all(&[0x53])?;
            }
            Instruction::I64LtU => {
                wr.write_all(&[0x54])?;
            }
            Instruction::I64GtS => {
                wr.write_all(&[0x55])?;
            }
            Instruction::I64GtU => {
                wr.write_all(&[0x56])?;
            }
            Instruction::I64LeS => {
                wr.write_all(&[0x57])?;
            }
            Instruction::I64LeU => {
                wr.write_all(&[0x58])?;
            }
            Instruction::I64GeS => {
                wr.write_all(&[0x59])?;
            }
            Instruction::I64GeU => {
                wr.write_all(&[0x5A])?;
            }
            Instruction::F32Eq => {
                wr.write_all(&[0x5B])?;
            }
            Instruction::F32Ne => {
                wr.write_all(&[0x5C])?;
            }
            Instruction::F32Lt => {
                wr.write_all(&[0x5D])?;
            }
            Instruction::F32Gt => {
                wr.write_all(&[0x5E])?;
            }
            Instruction::F32Le => {
                wr.write_all(&[0x5F])?;
            }
            Instruction::F32Ge => {
                wr.write_all(&[0x60])?;
            }
            Instruction::F64Eq => {
                wr.write_all(&[0x61])?;
            }
            Instruction::F64Ne => {
                wr.write_all(&[0x62])?;
            }
            Instruction::F64Lt => {
                wr.write_all(&[0x63])?;
            }
            Instruction::F64Gt => {
                wr.write_all(&[0x64])?;
            }
            Instruction::F64Le => {
                wr.write_all(&[0x65])?;
            }
            Instruction::F64Ge => {
                wr.write_all(&[0x66])?;
            }
            Instruction::I32Clz => {
                wr.write_all(&[0x67])?;
            }
            Instruction::I32Ctz => {
                wr.write_all(&[0x68])?;
            }
            Instruction::I32Popcnt => {
                wr.write_all(&[0x69])?;
            }
            Instruction::I32Add => {
                wr.write_all(&[0x6A])?;
            }
            Instruction::I32Sub => {
                wr.write_all(&[0x6B])?;
            }
            Instruction::I32Mul => {
                wr.write_all(&[0x6C])?;
            }
            Instruction::I32DivS => {
                wr.write_all(&[0x6D])?;
            }
            Instruction::I32DivU => {
                wr.write_all(&[0x6E])?;
            }
            Instruction::I32RemS => {
                wr.write_all(&[0x6F])?;
            }
            Instruction::I32RemU => {
                wr.write_all(&[0x70])?;
            }
            Instruction::I32And => {
                wr.write_all(&[0x71])?;
            }
            Instruction::I32Or => {
                wr.write_all(&[0x72])?;
            }
            Instruction::I32Xor => {
                wr.write_all(&[0x73])?;
            }
            Instruction::I32Shl => {
                wr.write_all(&[0x74])?;
            }
            Instruction::I32ShrS => {
                wr.write_all(&[0x75])?;
            }
            Instruction::I32ShrU => {
                wr.write_all(&[0x76])?;
            }
            Instruction::I32Rotl => {
                wr.write_all(&[0x77])?;
            }
            Instruction::I32Rotr => {
                wr.write_all(&[0x78])?;
            }
            Instruction::I64Clz => {
                wr.write_all(&[0x79])?;
            }
            Instruction::I64Ctz => {
                wr.write_all(&[0x7A])?;
            }
            Instruction::I64Popcnt => {
                wr.write_all(&[0x7B])?;
            }
            Instruction::I64Add => {
                wr.write_all(&[0x7C])?;
            }
            Instruction::I64Sub => {
                wr.write_all(&[0x7D])?;
            }
            Instruction::I64Mul => {
                wr.write_all(&[0x7E])?;
            }
            Instruction::I64DivS => {
                wr.write_all(&[0x7F])?;
            }
            Instruction::I64DivU => {
                wr.write_all(&[0x80])?;
            }
            Instruction::I64RemS => {
                wr.write_all(&[0x81])?;
            }
            Instruction::I64RemU => {
                wr.write_all(&[0x82])?;
            }
            Instruction::I64And => {
                wr.write_all(&[0x83])?;
            }
            Instruction::I64Or => {
                wr.write_all(&[0x84])?;
            }
            Instruction::I64Xor => {
                wr.write_all(&[0x85])?;
            }
            Instruction::I64Shl => {
                wr.write_all(&[0x86])?;
            }
            Instruction::I64ShrS => {
                wr.write_all(&[0x87])?;
            }
            Instruction::I64ShrU => {
                wr.write_all(&[0x88])?;
            }
            Instruction::I64Rotl => {
                wr.write_all(&[0x89])?;
            }
            Instruction::I64Rotr => {
                wr.write_all(&[0x8A])?;
            }
            Instruction::F32Abs => {
                wr.write_all(&[0x8B])?;
            }
            Instruction::F32Neg => {
                wr.write_all(&[0x8C])?;
            }
            Instruction::F32Ceil => {
                wr.write_all(&[0x8D])?;
            }
            Instruction::F32Floor => {
                wr.write_all(&[0x8E])?;
            }
            Instruction::F32Trunc => {
                wr.write_all(&[0x8F])?;
            }
            Instruction::F32Nearest => {
                wr.write_all(&[0x90])?;
            }
            Instruction::F32Sqrt => {
                wr.write_all(&[0x91])?;
            }
            Instruction::F32Add => {
                wr.write_all(&[0x92])?;
            }
            Instruction::F32Sub => {
                wr.write_all(&[0x93])?;
            }
            Instruction::F32Mul => {
                wr.write_all(&[0x94])?;
            }
            Instruction::F32Div => {
                wr.write_all(&[0x95])?;
            }
            Instruction::F32Min => {
                wr.write_all(&[0x96])?;
            }
            Instruction::F32Max => {
                wr.write_all(&[0x97])?;
            }
            Instruction::F32Copysign => {
                wr.write_all(&[0x98])?;
            }
            Instruction::F64Abs => {
                wr.write_all(&[0x99])?;
            }
            Instruction::F64Neg => {
                wr.write_all(&[0x9A])?;
            }
            Instruction::F64Ceil => {
                wr.write_all(&[0x9B])?;
            }
            Instruction::F64Floor => {
                wr.write_all(&[0x9C])?;
            }
            Instruction::F64Trunc => {
                wr.write_all(&[0x9D])?;
            }
            Instruction::F64Nearest => {
                wr.write_all(&[0x9E])?;
            }
            Instruction::F64Sqrt => {
                wr.write_all(&[0x9F])?;
            }
            Instruction::F64Add => {
                wr.write_all(&[0xA0])?;
            }
            Instruction::F64Sub => {
                wr.write_all(&[0xA1])?;
            }
            Instruction::F64Mul => {
                wr.write_all(&[0xA2])?;
            }
            Instruction::F64Div => {
                wr.write_all(&[0xA3])?;
            }
            Instruction::F64Min => {
                wr.write_all(&[0xA4])?;
            }
            Instruction::F64Max => {
                wr.write_all(&[0xA5])?;
            }
            Instruction::F64Copysign => {
                wr.write_all(&[0xA6])?;
            }
            Instruction::I32WrapI64 => {
                wr.write_all(&[0xA7])?;
            }
            Instruction::I32TruncF32S => {
                wr.write_all(&[0xA8])?;
            }
            Instruction::I32TruncF32U => {
                wr.write_all(&[0xA9])?;
            }
            Instruction::I32TruncF64S => {
                wr.write_all(&[0xAA])?;
            }
            Instruction::I32TruncF64U => {
                wr.write_all(&[0xAB])?;
            }
            Instruction::I64ExtendI32S => {
                wr.write_all(&[0xAC])?;
            }
            Instruction::I64ExtendI32U => {
                wr.write_all(&[0xAD])?;
            }
            Instruction::I64TruncF32S => {
                wr.write_all(&[0xAE])?;
            }
            Instruction::I64TruncF32U => {
                wr.write_all(&[0xAF])?;
            }
            Instruction::I64TruncF64S => {
                wr.write_all(&[0xB0])?;
            }
            Instruction::I64TruncF64U => {
                wr.write_all(&[0xB1])?;
            }
            Instruction::F32ConvertI32S => {
                wr.write_all(&[0xB2])?;
            }
            Instruction::F32ConvertI32U => {
                wr.write_all(&[0xB3])?;
            }
            Instruction::F32ConvertI64S => {
                wr.write_all(&[0xB4])?;
            }
            Instruction::F32ConvertI64U => {
                wr.write_all(&[0xB5])?;
            }
            Instruction::F32DemoteF64 => {
                wr.write_all(&[0xB6])?;
            }
            Instruction::F64ConvertI32S => {
                wr.write_all(&[0xB7])?;
            }
            Instruction::F64ConvertI32U => {
                wr.write_all(&[0xB8])?;
            }
            Instruction::F64ConvertI64S => {
                wr.write_all(&[0xB9])?;
            }
            Instruction::F64ConvertI64U => {
                wr.write_all(&[0xBA])?;
            }
            Instruction::F64PromoteF32 => {
                wr.write_all(&[0xBB])?;
            }
            Instruction::I32ReinterpretF32 => {
                wr.write_all(&[0xBC])?;
            }
            Instruction::I64ReinterpretF64 => {
                wr.write_all(&[0xBD])?;
            }
            Instruction::F32ReinterpretI32 => {
                wr.write_all(&[0xBE])?;
            }
            Instruction::F64ReinterpretI64 => {
                wr.write_all(&[0xBF])?;
            }
            Instruction::I32Extend8S => {
                wr.write_all(&[0xC0])?;
            }
            Instruction::I32Extend16S => {
                wr.write_all(&[0xC1])?;
            }
            Instruction::I64Extend8S => {
                wr.write_all(&[0xC2])?;
            }
            Instruction::I64Extend16S => {
                wr.write_all(&[0xC3])?;
            }
            Instruction::I64Extend32S => {
                wr.write_all(&[0xC4])?;
            }
            Instruction::I32TruncSatF32S => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(0)?;
            }
            Instruction::I32TruncSatF32U => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(1)?;
            }
            Instruction::I32TruncSatF64S => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(2)?;
            }
            Instruction::I32TruncSatF64U => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(3)?;
            }
            Instruction::I64TruncSatF32S => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(4)?;
            }
            Instruction::I64TruncSatF32U => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(5)?;
            }
            Instruction::I64TruncSatF64S => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(6)?;
            }
            Instruction::I64TruncSatF64U => {
                wr.write_all(&[0xFC])?;
                wr.write_u32(7)?;
            }
            Instruction::V128Load(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(0)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load8x8S(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(1)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load8x8U(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(2)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load16x4S(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(3)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load16x4U(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(4)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load32x2S(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(5)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load32x2U(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(6)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load8Splat(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(7)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load16Splat(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(8)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load32Splat(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(9)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load64Splat(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(10)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load32Zero(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(92)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load64Zero(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(93)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Store(ma) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(11)?;
                ma.write_into(wr)?;
            }
            Instruction::V128Load8Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(84)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Load16Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(85)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Load32Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(86)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Load64Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(87)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Store8Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(88)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Store16Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(89)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Store32Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(90)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Store64Lane(ma, li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(91)?;
                ma.write_into(wr)?;
                wr.write_u32(*li)?;
            }
            Instruction::V128Const(x) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(12)?;
                wr.write_all(&x.to_le_bytes())?;
            }
            Instruction::I8x16Shuffle(lis) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(13)?;
                for li in lis {
                    wr.write_u32(*li)?;
                }
            }
            Instruction::I8x16ExtractLaneS(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(21)?;
                wr.write_u32(*li)?;
            }
            Instruction::I8x16ExtractLaneU(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(22)?;
                wr.write_u32(*li)?;
            }
            Instruction::I8x16ReplaceLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(23)?;
                wr.write_u32(*li)?;
            }
            Instruction::I16x8ExtractLaneS(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(24)?;
                wr.write_u32(*li)?;
            }
            Instruction::I16x8ExtractLaneU(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(25)?;
                wr.write_u32(*li)?;
            }
            Instruction::I16x8ReplaceLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(26)?;
                wr.write_u32(*li)?;
            }
            Instruction::I32x4ExtractLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(27)?;
                wr.write_u32(*li)?;
            }
            Instruction::I32x4ReplaceLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(28)?;
                wr.write_u32(*li)?;
            }
            Instruction::I64x2ExtractLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(29)?;
                wr.write_u32(*li)?;
            }
            Instruction::I64x2ReplaceLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(30)?;
                wr.write_u32(*li)?;
            }
            Instruction::F32x4ExtractLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(31)?;
                wr.write_u32(*li)?;
            }
            Instruction::F32x4ReplaceLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(32)?;
                wr.write_u32(*li)?;
            }
            Instruction::F64x2ExtractLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(33)?;
                wr.write_u32(*li)?;
            }
            Instruction::F64x2ReplaceLane(li) => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(34)?;
                wr.write_u32(*li)?;
            }
            Instruction::I8X16Swizzle => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(14)?;
            }
            Instruction::I8x16Splat => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(15)?;
            }
            Instruction::I16x8Splat => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(16)?;
            }
            Instruction::I32x4Splat => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(17)?;
            }
            Instruction::I64x2Splat => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(18)?;
            }
            Instruction::F32x4Splat => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(19)?;
            }
            Instruction::F64x2Splat => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(20)?;
            }
            Instruction::I8x16Eq => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(35)?;
            }
            Instruction::I8x16Ne => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(36)?;
            }
            Instruction::I8X16LtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(37)?;
            }
            Instruction::I8X16LtU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(38)?;
            }
            Instruction::I8X16GtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(39)?;
            }
            Instruction::I8X16GtU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(40)?;
            }
            Instruction::I8X16LeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(41)?;
            }
            Instruction::I8X16LeU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(42)?;
            }
            Instruction::I8X16GeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(43)?;
            }
            Instruction::I8X16GeU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(44)?;
            }
            Instruction::I16x8Eq => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(45)?;
            }
            Instruction::I16x8Ne => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(46)?;
            }
            Instruction::I16x8LtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(47)?;
            }
            Instruction::I16x8LtU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(48)?;
            }
            Instruction::I16x8GtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(49)?;
            }
            Instruction::I16x8GtU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(50)?;
            }
            Instruction::I16x8LeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(51)?;
            }
            Instruction::I16x8LeU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(52)?;
            }
            Instruction::I16x8GeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(53)?;
            }
            Instruction::I16x8GeU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(54)?;
            }
            Instruction::I32x4Eq => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(55)?;
            }
            Instruction::I32x4Ne => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(56)?;
            }
            Instruction::I32x4LtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(57)?;
            }
            Instruction::I32x4LtU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(58)?;
            }
            Instruction::I32x4GtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(59)?;
            }
            Instruction::I32x4GtU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(60)?;
            }
            Instruction::I32x4LeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(61)?;
            }
            Instruction::I32x4LeU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(62)?;
            }
            Instruction::I32x4GeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(63)?;
            }
            Instruction::I32x4GeU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(64)?;
            }
            Instruction::I64x2Eq => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(214)?;
            }
            Instruction::I64x2Ne => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(215)?;
            }
            Instruction::I64x2LtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(216)?;
            }
            Instruction::I64x2GtS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(217)?;
            }
            Instruction::I64x2LeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(218)?;
            }
            Instruction::I64x2GeS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(219)?;
            }
            Instruction::F32x4Eq => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(65)?;
            }
            Instruction::F32x4Ne => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(66)?;
            }
            Instruction::F32x4Lt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(67)?;
            }
            Instruction::F32x4Gt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(68)?;
            }
            Instruction::F32x4Le => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(69)?;
            }
            Instruction::F32x4Ge => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(70)?;
            }
            Instruction::F64x2Eq => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(71)?;
            }
            Instruction::F64x2Ne => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(72)?;
            }
            Instruction::F64x2Lt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(73)?;
            }
            Instruction::F64x2Gt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(74)?;
            }
            Instruction::F64x2Le => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(75)?;
            }
            Instruction::F64x2Ge => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(76)?;
            }
            Instruction::V128Not => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(77)?;
            }
            Instruction::V128And => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(78)?;
            }
            Instruction::V128AndNot => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(79)?;
            }
            Instruction::V128Or => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(80)?;
            }
            Instruction::V128Xor => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(81)?;
            }
            Instruction::V128Bitselect => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(82)?;
            }
            Instruction::V128AnyTrue => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(83)?;
            }
            Instruction::I8x16Abs => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(96)?;
            }
            Instruction::I8x16Neg => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(97)?;
            }
            Instruction::I8x16Popcnt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(98)?;
            }
            Instruction::I8x16AllTrue => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(99)?;
            }
            Instruction::I8x16Bitmask => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(100)?;
            }
            Instruction::I8x16NarrowI16x8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(101)?;
            }
            Instruction::I8x16NarrowI16x8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(102)?;
            }
            Instruction::I8x16Shl => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(107)?;
            }
            Instruction::I8x16ShrS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(108)?;
            }
            Instruction::I8x16ShrU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(109)?;
            }
            Instruction::I8x16Add => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(110)?;
            }
            Instruction::I8x16AddSatS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(111)?;
            }
            Instruction::I8x16AddSatU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(112)?;
            }
            Instruction::I8x16Sub => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(113)?;
            }
            Instruction::I8x16SubSatS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(114)?;
            }
            Instruction::I8x16SubSatU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(115)?;
            }
            Instruction::I8x16MinS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(118)?;
            }
            Instruction::I8x16MinU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(119)?;
            }
            Instruction::I8x16MaxS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(120)?;
            }
            Instruction::I8x16MaxU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(121)?;
            }
            Instruction::I8x16AvgrU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(123)?;
            }
            Instruction::I16x8ExtAddPairwiseI8x16S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(124)?;
            }
            Instruction::I16x8ExtAddPairwiseI8x16U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(125)?;
            }
            Instruction::I16x8Abs => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(128)?;
            }
            Instruction::I16x8Neg => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(129)?;
            }
            Instruction::I16x8Q15MulrSatS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(130)?;
            }
            Instruction::I16x8AllTrue => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(131)?;
            }
            Instruction::I16x8Bitmask => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(132)?;
            }
            Instruction::I16x8NarrowI32x4S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(133)?;
            }
            Instruction::I16x8NarrowI32x4U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(134)?;
            }
            Instruction::I16x8ExtendLowI8X16S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(135)?;
            }
            Instruction::I16x8ExtendHighI8X16S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(136)?;
            }
            Instruction::I16x8ExtendLowI8X16U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(137)?;
            }
            Instruction::I16x8ExtendHighI8X16U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(138)?;
            }
            Instruction::I16x8Shl => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(139)?;
            }
            Instruction::I16x8ShrS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(140)?;
            }
            Instruction::I16x8ShrU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(141)?;
            }
            Instruction::I16x8Add => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(142)?;
            }
            Instruction::I16x8AddSatS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(143)?;
            }
            Instruction::I16x8AddSatU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(144)?;
            }
            Instruction::I16x8Sub => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(145)?;
            }
            Instruction::I16x8SubSatS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(146)?;
            }
            Instruction::I16x8SubSatU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(147)?;
            }
            Instruction::I16X8Mul => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(149)?;
            }
            Instruction::I16x8MinS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(150)?;
            }
            Instruction::I16x8MinU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(151)?;
            }
            Instruction::I16x8MaxS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(152)?;
            }
            Instruction::I16x8MaxU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(153)?;
            }
            Instruction::I16x8AvgrU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(155)?;
            }
            Instruction::I16x8ExtmulLowI8x16S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(156)?;
            }
            Instruction::I16x8ExtmulHighI8x16S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(157)?;
            }
            Instruction::I16x8ExtmulLowI8x16U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(158)?;
            }
            Instruction::I16x8ExtmulHighI8x16U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(159)?;
            }
            Instruction::I32x4ExtAddPairwiseI16x8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(126)?;
            }
            Instruction::I32x4ExtAddPairwiseI16x8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(127)?;
            }
            Instruction::I32x4Abs => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(160)?;
            }
            Instruction::I32x4Neg => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(161)?;
            }
            Instruction::I32x4AllTrue => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(163)?;
            }
            Instruction::I32x4Bitmask => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(164)?;
            }
            Instruction::I32x4ExtendLowI16X8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(167)?;
            }
            Instruction::I32x4ExtendHighI16X8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(168)?;
            }
            Instruction::I32x4ExtendLowI16X8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(169)?;
            }
            Instruction::I32x4ExtendHighI16X8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(170)?;
            }
            Instruction::I32x4Shl => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(171)?;
            }
            Instruction::I32x4ShrS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(172)?;
            }
            Instruction::I32x4ShrU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(173)?;
            }
            Instruction::I32x4Add => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(174)?;
            }
            Instruction::I32x4Sub => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(177)?;
            }
            Instruction::I32x4Mul => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(181)?;
            }
            Instruction::I32x4MinS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(182)?;
            }
            Instruction::I32x4MinU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(183)?;
            }
            Instruction::I32x4MaxS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(184)?;
            }
            Instruction::I32x4MaxU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(185)?;
            }
            Instruction::I32x4DotI16x8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(186)?;
            }
            Instruction::I32x4ExtmulLowI16x8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(188)?;
            }
            Instruction::I32x4ExtmulHighI16x8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(189)?;
            }
            Instruction::I32x4ExtmulLowI16x8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(190)?;
            }
            Instruction::I32x4ExtmulHighI16x8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(191)?;
            }
            Instruction::I64x2Abs => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(192)?;
            }
            Instruction::I64x2Neg => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(193)?;
            }
            Instruction::I64x2AllTrue => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(195)?;
            }
            Instruction::I64x2Bitmask => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(196)?;
            }
            Instruction::I64x2ExtendLowI16X8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(199)?;
            }
            Instruction::I64x2ExtendHighI16X8S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(200)?;
            }
            Instruction::I64x2ExtendLowI16X8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(201)?;
            }
            Instruction::I64x2ExtendHighI16X8U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(202)?;
            }
            Instruction::I64x2Shl => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(203)?;
            }
            Instruction::I64x2ShrS => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(204)?;
            }
            Instruction::I64x2ShrU => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(205)?;
            }
            Instruction::I64x2Add => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(206)?;
            }
            Instruction::I64x2Sub => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(209)?;
            }
            Instruction::I64x2Mul => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(213)?;
            }
            Instruction::I64x2ExtmulLowI32x4S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(220)?;
            }
            Instruction::I64x2ExtmulHighI32x4S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(221)?;
            }
            Instruction::I64x2ExtmulLowI32x4U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(222)?;
            }
            Instruction::I64x2ExtmulHighI32x4U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(223)?;
            }
            Instruction::F32x4Ceil => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(103)?;
            }
            Instruction::F32x4Floor => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(104)?;
            }
            Instruction::F32x4Trunc => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(105)?;
            }
            Instruction::F32x4Nearest => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(106)?;
            }
            Instruction::F32x4Abs => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(224)?;
            }
            Instruction::F32x4Neg => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(225)?;
            }
            Instruction::F32x4Sqrt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(227)?;
            }
            Instruction::F32x4Add => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(228)?;
            }
            Instruction::F32x4Sub => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(229)?;
            }
            Instruction::F32x4Mul => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(230)?;
            }
            Instruction::F32x4Div => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(231)?;
            }
            Instruction::F32x4Min => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(232)?;
            }
            Instruction::F32x4Max => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(233)?;
            }
            Instruction::F32x4Pmin => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(234)?;
            }
            Instruction::F32x4Pmax => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(235)?;
            }
            Instruction::F64x2Ceil => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(116)?;
            }
            Instruction::F64x2Floor => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(117)?;
            }
            Instruction::F64x2Trunc => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(122)?;
            }
            Instruction::F64x2Nearest => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(148)?;
            }
            Instruction::F64x2Abs => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(236)?;
            }
            Instruction::F64x2Neg => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(237)?;
            }
            Instruction::F64x2Sqrt => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(239)?;
            }
            Instruction::F64x2Add => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(240)?;
            }
            Instruction::F64x2Sub => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(241)?;
            }
            Instruction::F64x2Mul => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(242)?;
            }
            Instruction::F64x2Div => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(243)?;
            }
            Instruction::F64x2Min => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(244)?;
            }
            Instruction::F64x2Max => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(245)?;
            }
            Instruction::F64x2Pmin => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(246)?;
            }
            Instruction::F64x2Pmax => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(247)?;
            }
            Instruction::I32x4TruncSatF32x4S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(248)?;
            }
            Instruction::I32x4TruncSatF32x4U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(249)?;
            }
            Instruction::F32x4ConvertI32x4S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(250)?;
            }
            Instruction::F32x4ConvertI32x4U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(251)?;
            }
            Instruction::I32x4TruncSatF64x2SZero => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(252)?;
            }
            Instruction::I32x4TruncSatF64x2UZero => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(253)?;
            }
            Instruction::F64x2ConvertLowI32x4S => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(254)?;
            }
            Instruction::F64x2ConvertLowI32x4U => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(255)?;
            }
            Instruction::F32x4DemoteF64x2Zero => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(94)?;
            }
            Instruction::F64x2PromoteLowF32x4 => {
                wr.write_all(&[0xFD])?;
                wr.write_u32(95)?;
            }
        }
        Ok(())
    }

    pub(crate) fn write_slice_into(this: &[Self], wr: &mut impl Write) -> Result<(), io::Error> {
        for x in this {
            x.write_into(wr)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Expression(Vec<Instruction>);

impl Expression {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        log::trace!("expression from bytes: start reading instructions");
        let (instrs, _, bytes) = Instruction::from_bytes_vec(bytes, &[0x0B])?;

        Ok((Self(instrs), bytes))
    }

    pub(crate) fn write_into(&self, wr: &mut impl Write) -> Result<(), io::Error> {
        Instruction::write_slice_into(&self.0, wr)
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.0
    }
}
