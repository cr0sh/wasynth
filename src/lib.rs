pub mod instructions;
pub mod parse;
pub mod synth;
pub mod wasm_types;

use std::marker::PhantomData;

use log::trace;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected end of file: expected {0} bytes, got {1}")]
    UnexpectedEof(usize, usize),
    #[error("trailing bytes")]
    TrailingBytes, // NOTE: this can happen while parsing in a middle of a payload if a size to parse is specified
    #[error("invalid magic: expected 0x0061736d(\\x00asm), got 0x{0:02x}{1:02x}{2:02x}{3:02x}")]
    Magic(u8, u8, u8, u8),
    #[error("unsupported WebAssembly version {0}")]
    UnsupportedVersion(u32),
    #[error("cannot parse name as UTF-8")]
    ParseName(#[source] std::str::Utf8Error),
    #[error("cannot read number as LEB128")]
    ReadLeb128(#[source] leb128::read::Error),
    #[error("invalid section ID {0}")]
    SectionID(u8),
    #[error("invalid function type ID 0x{0:02x}, expected 0x60")]
    FunctionTypeId(u8),
    #[error("invalid reference type ID 0x{0:02x}")]
    ReferenceTypeId(u8),
    #[error("invalid value type ID 0x{0:02x}")]
    ValueTypeId(u8),
    #[error("invalid limits tag 0x{0:02x}")]
    LimitsTag(u8),
    #[error("invalid global type mutability 0x{0:02x}")]
    GlobalTypeMutability(u8),
    #[error("invalid import description tag 0x{0:02x}")]
    ImportDescriptionTag(u8),
    #[error("invalid 0xFC instruction subopcode {0}")]
    HexFcInstructionSubopcode(u32),
    #[error("invalid vector instruction subopcode {0}")]
    VectorInstructionSubopcode(u32),
    #[error("invalid memory instruction 0x{0:02x} 0x{1:02x}")]
    MemoryInstruction(u8, u8),
    #[error("{instr} instruction shoud have a trailing zero byte, got {byte}")]
    MemoryInstructionNoTrailingZero { instr: &'static str, byte: u8 },
    #[error("invalid WebAssembly opcode 0x{0:02x}")]
    Opcode(u8),
    #[error("invalid data section tag {0}")]
    DataSectionTag(u32),
}

/// Convenince trait for reading bytes.
pub(crate) trait Bytes: Sized {
    type ArrayRef<const N: usize>;
    type VectorIterator<T, F: FnMut(Self) -> Result<(T, Self), Error>>: Iterator<
        Item = Result<T, Error>,
    >;
    type NameRef;

    /// Reads `N` bytes into an array with length `N`.
    fn advance<const N: usize>(self) -> Result<(Self::ArrayRef<N>, Self), Error>;
    /// Reads `len` bytes into an slice with length `len`.
    fn advance_slice(self, len: usize) -> Result<(Self, Self), Error>;
    /// Reads 4 bytes into an u32 value.
    fn advance_u32(self) -> Result<(u32, Self), Error>;
    /// Reads 8 bytes into an u64 value.
    fn advance_u64(self) -> Result<(u64, Self), Error>;
    /// Reads 4 bytes into an i32 value.
    fn advance_s32(self) -> Result<(i32, Self), Error>;
    /// Reads 8 bytes into an i64 value.
    fn advance_s64(self) -> Result<(i64, Self), Error>;
    /// Reads 4 bytes into an f32 value.
    fn advance_f32(self) -> Result<(f32, Self), Error>;
    /// Reads 8 bytes into an f64 value.
    fn advance_f64(self) -> Result<(f64, Self), Error>;
    /// TODO: docs
    fn advance_vector<T, F: FnMut(Self) -> Result<(T, Self), Error>>(
        self,
        func: F,
    ) -> Result<Self::VectorIterator<T, F>, Error>;
    /// Reads a WebAssembly spec-defined 'name'(which is a length byte followed by UTF-8 bytes) with `advance_vector'.
    fn advance_name(self) -> Result<(Self::NameRef, Self), Error>;
}

impl<'a> Bytes for &'a [u8] {
    type ArrayRef<const N: usize> = &'a [u8; N];
    type VectorIterator<T, F: FnMut(Self) -> Result<(T, Self), Error>> = VectorIterator<'a, T, F>;
    type NameRef = &'a str;

    fn advance<const N: usize>(self) -> Result<(Self::ArrayRef<N>, Self), Error> {
        trace!("advance<{N}>");
        if self.len() < N {
            Err(Error::UnexpectedEof(N, self.len()))
        } else {
            let (x, y) = self.split_at(N);
            // SAFETY: x points to [T; N]? Yes it's [T] of length N (checked by split_at)
            Ok((unsafe { &*(x.as_ptr() as *const [u8; N]) }, y))
        }
    }

    fn advance_slice(self, len: usize) -> Result<(Self, Self), Error> {
        trace!("advance_slice({len})");
        if self.len() < len {
            Err(Error::UnexpectedEof(len, self.len()))
        } else {
            let (x, y) = self.split_at(len);
            Ok((x, y))
        }
    }

    fn advance_u32(self) -> Result<(u32, Self), Error> {
        trace!("advance_u32");
        let advance_len = self.len().min(5);
        let mut head = &self[0..advance_len];
        let x = leb128::read::unsigned(&mut head).map_err(Error::ReadLeb128)?;

        if x > u32::MAX as u64 {
            return Err(Error::ReadLeb128(leb128::read::Error::Overflow));
        }

        let (_, this) = self.advance_slice(advance_len - head.len()).unwrap();
        Ok((x as u32, this))
    }

    fn advance_u64(self) -> Result<(u64, Self), Error> {
        trace!("advance_u64");
        let advance_len = self.len().min(10);
        let mut head = &self[0..advance_len];
        let x = leb128::read::unsigned(&mut head).map_err(Error::ReadLeb128)?;

        let (_, this) = self.advance_slice(advance_len - head.len()).unwrap();
        Ok((x, this))
    }

    fn advance_s32(self) -> Result<(i32, Self), Error> {
        trace!("advance_s32");
        let advance_len = self.len().min(5);
        let mut head = &self[0..advance_len];
        let x = leb128::read::signed(&mut head).map_err(Error::ReadLeb128)?;

        if x > i32::MAX as i64 || x < i32::MIN as i64 {
            return Err(Error::ReadLeb128(leb128::read::Error::Overflow));
        }

        let (_, this) = self.advance_slice(advance_len - head.len()).unwrap();
        Ok((x as i32, this))
    }

    fn advance_s64(self) -> Result<(i64, Self), Error> {
        trace!("advance_s64");
        let advance_len = self.len().min(10);
        let mut head = &self[0..advance_len];
        let x = leb128::read::signed(&mut head).map_err(Error::ReadLeb128)?;

        let (_, this) = self.advance_slice(advance_len - head.len()).unwrap();
        Ok((x, this))
    }

    fn advance_f32(self) -> Result<(f32, Self), Error> {
        trace!("advance_f32");
        let (f, this) = self.advance()?;
        Ok((f32::from_le_bytes(*f), this))
    }

    fn advance_f64(self) -> Result<(f64, Self), Error> {
        trace!("advance_f64");
        let (f, this) = self.advance()?;
        Ok((f64::from_le_bytes(*f), this))
    }

    fn advance_vector<T, F: FnMut(Self) -> Result<(T, Self), Error>>(
        self,
        func: F,
    ) -> Result<Self::VectorIterator<T, F>, Error> {
        trace!("advance_vector (enter)");
        let (n, this) = self.advance_u32()?;
        trace!("vector elements = {n}");

        Ok(VectorIterator {
            bytes: this,
            count: n.try_into().expect("vector length overflow"),
            func,
            _phantom: PhantomData,
        })
    }

    fn advance_name(self) -> Result<(Self::NameRef, Self), Error> {
        trace!("advance_name");
        let (&[n], this) = self.advance()?;
        let (bytes, this) = this.advance_slice(n as usize)?;
        Ok((std::str::from_utf8(bytes).map_err(Error::ParseName)?, this))
    }
}

/// Iterator of vector elements returned by [`Bytes::advance_vector`].
struct VectorIterator<'bytes, T, F> {
    bytes: &'bytes [u8],
    count: usize,
    func: F,
    _phantom: PhantomData<T>,
}

impl<'bytes, T, F> Iterator for VectorIterator<'bytes, T, F>
where
    F: FnMut(&'bytes [u8]) -> Result<(T, &'bytes [u8]), Error>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            match (self.func)(self.bytes) {
                Ok((x, rest)) => {
                    self.bytes = rest;
                    Some(Ok(x))
                }
                Err(e) => Some(Err(e)),
            }
        } else {
            trace!("advance_vector (iterator end)");
            None
        }
    }
}

impl<'bytes, T, F> VectorIterator<'bytes, T, F> {
    /// Transforms this iterator into remaining bytes slice.
    pub fn finalize(self) -> &'bytes [u8] {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use crate::Bytes;

    #[test]
    fn test_advance() {
        let bytes = "\x03\x02\x01\x02".as_bytes();
        let ([n], bytes) = bytes.advance().unwrap();
        assert_eq!(*n, 3);
        let ([a, b, c], bytes) = bytes.advance().unwrap();
        assert_eq!(*a, 2);
        assert_eq!(*b, 1);
        assert_eq!(*c, 2);

        assert_eq!(bytes, &[]);
    }

    #[test]
    fn test_advance_slice() {
        let bytes = "\x03\x02\x01\x02".as_bytes();
        let (n, bytes) = bytes.advance_slice(1).unwrap();
        assert_eq!(n, &[3]);
        let (n, bytes) = bytes.advance_slice(3).unwrap();
        assert_eq!(n, &[2, 1, 2]);

        assert_eq!(bytes, &[]);
    }

    quickcheck! {
        fn test_advance_u32(x: u32) -> bool {
            let mut buf = Vec::new();
            leb128::write::unsigned(&mut buf, x as u64).unwrap();
            let (y, rest) = buf.advance_u32().unwrap();
            x == y && rest.is_empty()
        }

        fn test_advance_u64(x: u64) -> bool {
            let mut buf = Vec::new();
            leb128::write::unsigned(&mut buf, x).unwrap();
            let (y, rest) = buf.advance_u64().unwrap();
            x == y && rest.is_empty()
        }

        fn test_advance_i32(x: i32) -> bool {
            let mut buf = Vec::new();
            leb128::write::signed(&mut buf, x as i64).unwrap();
            let (y, rest) = buf.advance_s32().unwrap();
            x == y && rest.is_empty()
        }

        fn test_advance_i64(x: i64) -> bool {
            let mut buf = Vec::new();
            leb128::write::signed(&mut buf, x).unwrap();
            let (y, rest) = buf.advance_s64().unwrap();
            x == y && rest.is_empty()
        }

        fn test_advance_f32(x: f32) -> bool {
            use std::io::Write;
            let mut buf = Vec::new();
            buf.write_all(&x.to_le_bytes()).unwrap();
            let (y, rest) = buf.advance_f32().unwrap();
            (if x.is_nan() {
                y.is_nan()
            } else {
                x == y
            }) && rest.is_empty()
        }

        fn test_advance_f64(x: f64) -> bool {
            use std::io::Write;
            let mut buf = Vec::new();
            buf.write_all(&x.to_le_bytes()).unwrap();
            let (y, rest) = buf.advance_f64().unwrap();
            (if x.is_nan() {
                y.is_nan()
            } else {
                x == y
            }) && rest.is_empty()
        }
    }

    #[test]
    fn test_advance_vector() {
        let bytes = "\x03\x02\x01\x02".as_bytes();
        let mut sum = 0;
        let mut numbers = bytes
            .advance_vector(|x| {
                let (&[n], x) = x.advance()?;
                Ok((n, x))
            })
            .unwrap();

        for number in &mut numbers {
            sum += number.unwrap();
        }
        assert_eq!(sum, 5);
        assert_eq!(numbers.finalize(), &[]);
    }

    #[test]
    fn test_advance_name() {
        let bytes = "\x04wasm".as_bytes();
        let (name, bytes) = bytes.advance_name().unwrap();
        assert_eq!(name, "wasm");
        assert_eq!(bytes, &[]);
    }
}
