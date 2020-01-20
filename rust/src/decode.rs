use std::io;
use std::collections::BTreeMap;
use std::fmt;
use std::str;

use std::convert::TryInto;

use core::cmp::Ordering;

use byteorder::{BigEndian, ReadBytesExt};

#[doc(hidden)]
// Wrapper over floats to allow them being key in the map
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Float(f64);

impl core::cmp::Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 { return Ordering::Less }
        if self.0 > other.0 { return Ordering::Greater }

        Ordering::Equal
    }
}

impl core::cmp::Eq for Float {}

/// Internal representation of decoded data
#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum Type {
    #[doc(hidden)]
    Byte(u8),
    #[doc(hidden)]
    Integer(i32),
    #[doc(hidden)]
    BigInt {
        sign: bool,
        repr: Vec<u8>
    },
    #[doc(hidden)]
    Float(Float),
    #[doc(hidden)]
    Tuple(Vec<Type>),
    #[doc(hidden)]
    List(Vec<Type>),
    #[doc(hidden)]
    Binary(Vec<u8>),
    #[doc(hidden)]
    Map(BTreeMap<Type, Type>)
}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Type::Byte(b) => write!(fmt, "{}", b),
            Type::Integer(n) => write!(fmt, "{}", n),
            Type::Float(f) => write!(fmt, "{}", f.0),
            Type::Tuple(ref data) => fmt.debug_set().entries(data.into_iter()).finish(),
            Type::List(ref data) => fmt.debug_list().entries(data.into_iter()).finish(),
            Type::Binary(ref data) => {
                match str::from_utf8(data) {
                    Ok(s) => write!(fmt, "<<{:?}>>", s),
                    _ => write!(fmt, "<<{:?}>>", data)
                }
            }
            Type::Map(ref map) => fmt.debug_map().entries(map.into_iter()).finish(),
            _ => std::unimplemented!("Not yet implemented")
        }
    }
}

impl Type {
    /// Checks if current value is an integer
    pub fn is_integer(&self) -> bool {
        match *self {
            Type::Byte(_) => true,
            Type::Integer(_) => true,
            Type::BigInt { .. } => true,
            _ => false
        }
    }

    /// Checks if current value is a float
    pub fn is_float(&self) -> bool {
        match *self {
            Type::Float(_) => true,
            _ => false
        }
    }

    /// Checks if current value is a numeric value (either integer or float)
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    /// Checks if current value is a tuple
    pub fn is_tuple(&self) -> bool {
        match *self {
            Type::Tuple(_) => true,
            _ => false
        }
    }

    /// Checks if current value is a list
    pub fn is_list(&self) -> bool {
        match *self {
            Type::List(_) => true,
            _ => false
        }
    }

    /// Checks if current value is a binary
    pub fn is_binary(&self) -> bool {
        match *self {
            Type::Binary(_) => true,
            _ => false
        }
    }

    /// Checks if current value is a map
    pub fn is_map(&self) -> bool {
        match *self {
            Type::Map(_) => true,
            _ => false
        }
    }

    /// Checks if current value is a charlist (aka Erlang string)
    pub fn is_charlist(&self) -> bool {
        fn valid_codepoint(t: &Type) -> bool {
            match *t {
                Type::Byte(_) => true,
                Type::Integer(n) if n <= 0x10FFFF => true,
                _ => false
            }
        }

        match *self {
            Type::List(ref data) => data.into_iter().all(valid_codepoint),
            _ => false
        }
    }
}

pub fn decode_type(reader: &mut impl io::Read) -> io::Result<Type> {
    let mut header = [0u8; 1];

    reader.read_exact(&mut header)?;

    match header[0] {
        97 => reader.read_u8().map(Type::Byte),
        98 => reader.read_i32::<BigEndian>().map(Type::Integer),
        70 => reader.read_f64::<BigEndian>().and_then(validate_float).map(Type::Float),
        104 => read_small_tuple(reader).map(Type::Tuple),
        105 => read_big_tuple(reader).map(Type::Tuple),
        106 => Ok(Type::List(vec![])),
        107 => read_bytelist(reader).map(Type::List),
        108 => read_list(reader).map(Type::List),
        109 => read_binary(reader).map(Type::Binary),
        110 => std::unimplemented!("Small arbitrary integer deserialization is currently unsupported"),
        111 => std::unimplemented!("Big arbitrary integer deserialization is currently unsupported"),
        116 => read_map(reader).map(Type::Map),
        sig => std::unimplemented!("{:x?} is unknown", sig)
    }
}

fn validate_float(float: f64) -> io::Result<Float> {
    if float.is_nan() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Found NaN in float field"))
    }
    if float.is_infinite() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Found Infinity in float field"))
    }

    Ok(Float(float))
}

fn read_small_tuple(reader: &mut impl io::Read) -> io::Result<Vec<Type>> {
    let size = reader.read_u8()? as usize;

    read_vector(size, reader)
}

fn read_big_tuple(reader: &mut impl io::Read) -> io::Result<Vec<Type>> {
    let size = reader.read_u32::<BigEndian>()? as usize;

    read_vector(size, reader)
}

fn read_list(reader: &mut impl io::Read) -> io::Result<Vec<Type>> {
    let size = reader.read_u32::<BigEndian>()? as usize;

    let vec = read_vector(size, reader)?;

    if reader.read_u8()? == 106 {
        Ok(vec)
    } else {
        panic!("Improper list");
    }
}

fn read_binary(reader: &mut impl io::Read) -> io::Result<Vec<u8>> {
    let size = reader.read_u32::<BigEndian>()? as usize;
    let mut data = vec![0; size];

    reader.read_exact(&mut data)?;

    Ok(data)
}

fn read_bytelist(reader: &mut impl io::Read) -> io::Result<Vec<Type>> {
    let size = reader.read_u16::<BigEndian>()? as usize;
    let mut data = vec![0; size];

    reader.read_exact(&mut data)?;

    Ok(data.into_iter().map(Type::Byte).collect())
}

fn read_vector(size: usize, reader: &mut impl io::Read) -> io::Result<Vec<Type>> {
    let mut data = Vec::with_capacity(size);

    for _ in 0..size {
        let elem = decode_type(reader)?;
        data.push(elem);
    }

    Ok(data)
}

fn read_map(reader: &mut impl io::Read) -> io::Result<BTreeMap<Type, Type>> {
    let size = reader.read_u32::<BigEndian>()? as usize;
    let mut map = BTreeMap::new();

    for _ in 0..size {
        let key = decode_type(reader)?;
        let value = decode_type(reader)?;

        map.insert(key, value);
    }

    Ok(map)
}

#[derive(Debug)]
pub enum TryFromErnieError {
    InvalidType(Type),
    InvalidTupleArity(usize),
    NotUtf8(std::string::FromUtf8Error)
}


impl TryInto<u8> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Type::Byte(b) => Ok(b),
            Type::Integer(n) if n >= core::u8::MIN as i32 && n <= core::u8::MAX as i32 => Ok(n as u8),
            Type::BigInt { sign, ref repr } if !sign && repr.len() == 1 => Ok(repr[0]),
            Type::BigInt { ref repr, .. } if repr.len() == 0 => Ok(0),
            _ => Err(TryFromErnieError::InvalidType(self))
        }
    }
}

impl TryInto<i32> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Type::Byte(b) => Ok(b as i32),
            Type::Integer(n) => Ok(n),
            Type::BigInt { sign, ref repr } if !sign && repr.len() == 1 => Ok(repr[0] as i32),
            Type::BigInt { sign, ref repr } if !sign && repr.len() == 2 => Ok(i32::from_be_bytes([repr[0], repr[1], 0, 0])),
            Type::BigInt { sign, ref repr } if !sign && repr.len() == 3 => Ok(i32::from_be_bytes([repr[0], repr[1], repr[2], 0])),
            Type::BigInt { sign, ref repr } if !sign && repr.len() == 4 && repr[3] < 0x80 => Ok(i32::from_be_bytes([repr[0], repr[1], repr[2], repr[4]])),
            Type::BigInt { sign, ref repr } if sign && repr.len() == 1 => Ok(-(repr[0] as i32)),
            Type::BigInt { sign, ref repr } if sign && repr.len() == 2 => Ok(-i32::from_be_bytes([repr[0], repr[1], 0, 0])),
            Type::BigInt { sign, ref repr } if sign && repr.len() == 3 => Ok(-i32::from_be_bytes([repr[0], repr[1], repr[2], 0])),
            Type::BigInt { sign, ref repr } if sign && repr.len() == 4 && repr[3] < 0x80 => Ok(-i32::from_be_bytes([repr[0], repr[1], repr[2], repr[4]])),
            Type::BigInt { ref repr, .. } if repr.len() == 0 => Ok(0),
            _ => Err(TryFromErnieError::InvalidType(self))
        }
    }
}

impl TryInto<f64> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Type::Float(f) => Ok(f.0),
            _ => Err(TryFromErnieError::InvalidType(self))
        }
    }
}

impl TryInto<Vec<Type>> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<Vec<Type>, Self::Error> {
        match self {
            Type::Tuple(data) => Ok(data),
            Type::List(data) => Ok(data),
            _ => Err(TryFromErnieError::InvalidType(self))
        }
    }
}

impl TryInto<Vec<u8>> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Type::Binary(data) => Ok(data),
            _ => Err(TryFromErnieError::InvalidType(self))
        }
    }
}

impl TryInto<String> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<String, Self::Error> {
        self.try_into()
            .and_then(|vec| String::from_utf8(vec).map_err(TryFromErnieError::NotUtf8))
    }
}

impl TryInto<BTreeMap<Type, Type>> for Type {
    type Error = TryFromErnieError;

    fn try_into(self) -> Result<BTreeMap<Type, Type>, Self::Error> {
        match self {
            Type::Map(data) => Ok(data),
            _ => Err(TryFromErnieError::InvalidType(self))
        }
    }
}
