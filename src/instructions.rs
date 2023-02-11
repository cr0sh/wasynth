use crate::{
    wasm_types::{ReferenceType, ValueType},
    Bytes, Error,
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
                let (instrs, _, bytes) = Self::from_bytes_vec(bytes, &[0x0b])?;
                Ok((Self::Block(bt, instrs), bytes))
            }
            0x03 => {
                let (bt, bytes) = BlockType::from_bytes(bytes)?;
                let (instrs, _, bytes) = Self::from_bytes_vec(bytes, &[0x0b])?;
                Ok((Self::Loop(bt, instrs), bytes))
            }
            0x04 => {
                let (bt, bytes) = BlockType::from_bytes(bytes)?;
                let (instrs, end, bytes) = Self::from_bytes_vec(bytes, &[0x05, 0x0b])?;
                match end {
                    0x05 => {
                        let (elseinstrs, _, bytes) = Self::from_bytes_vec(bytes, &[0x0b])?;
                        Ok((Self::If(bt, instrs, Some(elseinstrs)), bytes))
                    }
                    0x0b => Ok((Self::If(bt, instrs, None), bytes)),
                    _ => unreachable!(),
                }
            }
            0x0c => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::Br(li), bytes))
            }
            0x0d => {
                let (li, bytes) = bytes.advance_u32()?;
                Ok((Self::BrIf(li), bytes))
            }
            0x0e => {
                let mut liit = bytes.advance_vector(<&[u8]>::advance_u32)?;
                let mut lis = Vec::new();
                for li in &mut liit {
                    lis.push(li?);
                }
                let bytes = liit.finalize();
                let (ln, bytes) = bytes.advance_u32()?;
                Ok((Self::BrTable(lis, ln), bytes))
            }
            0x0f => Ok((Self::Return, bytes)),
            0x10 => {
                let (fi, bytes) = bytes.advance_u32()?;
                Ok((Self::Call(fi), bytes))
            }
            0x11 => {
                let (ty, bytes) = bytes.advance_u32()?;
                let (table, bytes) = bytes.advance_u32()?;
                Ok((Self::CallIndirect { ty, table }, bytes))
            }
            0xd0 => {
                let (&[rt], bytes) = bytes.advance()?;
                let rt = ReferenceType::from_byte(rt)?;
                Ok((Self::RefNull(rt), bytes))
            }
            0xd1 => Ok((Self::RefIsNull, bytes)),
            0xd2 => {
                let (fi, bytes) = bytes.advance_u32()?;
                Ok((Self::RefFunc(fi), bytes))
            }
            0x1a => Ok((Self::Drop, bytes)),
            0x1b => Ok((Self::SelectNumeric, bytes)),
            0x1c => {
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
            0x3a => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Store8(ma), bytes))
            }
            0x3b => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I32Store16(ma), bytes))
            }
            0x3c => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store8(ma), bytes))
            }
            0x3d => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store16(ma), bytes))
            }
            0x3e => {
                let (ma, bytes) = MemArg::from_bytes(bytes)?;
                Ok((Self::I64Store32(ma), bytes))
            }
            0x3f => {
                let (&[zero], bytes) = bytes.advance()?;
                if zero != 0x00 {
                    return Err(Error::MemoryInstruction(0x3f, zero));
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
            0x4a => Ok((Self::I32GtS, bytes)),
            0x4b => Ok((Self::I32GtU, bytes)),
            0x4c => Ok((Self::I32LeS, bytes)),
            0x4d => Ok((Self::I32LeU, bytes)),
            0x4e => Ok((Self::I32GeS, bytes)),
            0x4f => Ok((Self::I32GeU, bytes)),
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
            0x5a => Ok((Self::I64GeU, bytes)),
            0x5b => Ok((Self::F32Eq, bytes)),
            0x5c => Ok((Self::F32Ne, bytes)),
            0x5d => Ok((Self::F32Lt, bytes)),
            0x5e => Ok((Self::F32Gt, bytes)),
            0x5f => Ok((Self::F32Le, bytes)),
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
            0x6a => Ok((Self::I32Add, bytes)),
            0x6b => Ok((Self::I32Sub, bytes)),
            0x6c => Ok((Self::I32Mul, bytes)),
            0x6d => Ok((Self::I32DivS, bytes)),
            0x6e => Ok((Self::I32DivU, bytes)),
            0x6f => Ok((Self::I32RemS, bytes)),
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
            0x7a => Ok((Self::I64Ctz, bytes)),
            0x7b => Ok((Self::I64Popcnt, bytes)),
            0x7c => Ok((Self::I64Add, bytes)),
            0x7d => Ok((Self::I64Sub, bytes)),
            0x7e => Ok((Self::I64Mul, bytes)),
            0x7f => Ok((Self::I64DivS, bytes)),
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
            0x8a => Ok((Self::I64Rotr, bytes)),
            0x8b => Ok((Self::F32Abs, bytes)),
            0x8c => Ok((Self::F32Neg, bytes)),
            0x8d => Ok((Self::F32Ceil, bytes)),
            0x8e => Ok((Self::F32Floor, bytes)),
            0x8f => Ok((Self::F32Trunc, bytes)),
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
            0x9a => Ok((Self::F64Neg, bytes)),
            0x9b => Ok((Self::F64Ceil, bytes)),
            0x9c => Ok((Self::F64Floor, bytes)),
            0x9d => Ok((Self::F64Trunc, bytes)),
            0x9e => Ok((Self::F64Nearest, bytes)),
            0x9f => Ok((Self::F64Sqrt, bytes)),
            0xa0 => Ok((Self::F64Add, bytes)),
            0xa1 => Ok((Self::F64Sub, bytes)),
            0xa2 => Ok((Self::F64Mul, bytes)),
            0xa3 => Ok((Self::F64Div, bytes)),
            0xa4 => Ok((Self::F64Min, bytes)),
            0xa5 => Ok((Self::F64Max, bytes)),
            0xa6 => Ok((Self::F64Copysign, bytes)),
            0xa7 => Ok((Self::I32WrapI64, bytes)),
            0xa8 => Ok((Self::I32TruncF32S, bytes)),
            0xa9 => Ok((Self::I32TruncF32U, bytes)),
            0xaa => Ok((Self::I32TruncF64S, bytes)),
            0xab => Ok((Self::I32TruncF64U, bytes)),
            0xac => Ok((Self::I64ExtendI32S, bytes)),
            0xad => Ok((Self::I64ExtendI32U, bytes)),
            0xae => Ok((Self::I64TruncF32S, bytes)),
            0xaf => Ok((Self::I64TruncF32U, bytes)),
            0xb0 => Ok((Self::I64TruncF64S, bytes)),
            0xb1 => Ok((Self::I64TruncF64U, bytes)),
            0xb2 => Ok((Self::F32ConvertI32S, bytes)),
            0xb3 => Ok((Self::F32ConvertI32U, bytes)),
            0xb4 => Ok((Self::F32ConvertI64S, bytes)),
            0xb5 => Ok((Self::F32ConvertI64U, bytes)),
            0xb6 => Ok((Self::F32DemoteF64, bytes)),
            0xb7 => Ok((Self::F64ConvertI32S, bytes)),
            0xb8 => Ok((Self::F64ConvertI32U, bytes)),
            0xb9 => Ok((Self::F64ConvertI64S, bytes)),
            0xba => Ok((Self::F64ConvertI64U, bytes)),
            0xbb => Ok((Self::F64PromoteF32, bytes)),
            0xbc => Ok((Self::I32ReinterpretF32, bytes)),
            0xbd => Ok((Self::I64ReinterpretF64, bytes)),
            0xbe => Ok((Self::F32ReinterpretI32, bytes)),
            0xbf => Ok((Self::F64ReinterpretI64, bytes)),
            0xc0 => Ok((Self::I32Extend8S, bytes)),
            0xc1 => Ok((Self::I32Extend16S, bytes)),
            0xc2 => Ok((Self::I64Extend8S, bytes)),
            0xc3 => Ok((Self::I64Extend16S, bytes)),
            0xc4 => Ok((Self::I64Extend32S, bytes)),
            0xfc => {
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
            0xFD => todo!("vector instructions"),
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
}

#[derive(Clone, Debug)]
pub struct Expression(Vec<Instruction>);

impl Expression {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        log::trace!("expression from bytes: start reading instructions");
        let (instrs, _, bytes) = Instruction::from_bytes_vec(bytes, &[0x0b])?;

        Ok((Self(instrs), bytes))
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.0
    }
}
