use std::io;
use std::collections::{HashMap, BTreeMap};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

pub type Result = io::Result<()>;

fn write_int(sign: bool, bytes: &[u8], out: &mut impl io::Write) -> io::Result<()> {
    let mut size = bytes.len();

    while size > 1 && bytes[size - 1] == 0 { size -= 1; }

    let sign = if sign { 1 } else { 0 };

    match size {
        1 => out.write_u8(97)?,
        2..=3 => {
            out.write_u8(98)?;
            size = 4;
        }
        4..=255 => out.write_all(&[110, size as u8, sign])?,
        size if size <= std::u16::MAX as usize => {
            out.write_u8(111)?;
            out.write_u16::<BigEndian>(size as u16)?;
            out.write_u8(sign)?;
        }
        _ => std::unimplemented!()
    }

    out.write_all(&bytes[..size])
}

pub trait Encode {
    fn encode(&self, out: &mut impl io::Write) -> Result;
}

impl Encode for u8 {
    fn encode(&self, out: &mut impl io::Write) -> Result { out.write_all(&[97, *self]) }
}

impl Encode for i8 {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if *self > 0 {
            (*self as u8).encode(out)
        } else {
            (*self as i32).encode(out)
        }
    }
}

impl Encode for i32 {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if *self <= 255 {
            (*self as u8).encode(out)
        } else {
            out.write_u8(98)?;
            out.write_i32::<BigEndian>(*self)
        }
    }
}

impl Encode for u32 {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if *self <= std::i32::MAX as u32 {
            (*self as i32).encode(out)
        } else {
            (*self as u64).encode(out)
        }
    }
}

impl Encode for u64 {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if *self <= std::i32::MAX as u64 {
            (*self as i32).encode(out)
        } else {
            let mut tmp = [0u8; 8];
            let mut w = &mut tmp[..];
            w.write_u64::<LittleEndian>(*self)?;
            write_int(false, &tmp, out)
        }
    }
}

impl Encode for i64 {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if *self <= std::i32::MAX as i64 && *self >= std::i32::MIN as i64 {
            (*self as i32).encode(out)
        } else {
            let mut tmp = [0u8; 8];
            let mut w = &mut tmp[..];
            w.write_i64::<LittleEndian>(self.abs())?;
            write_int(*self < 0, &tmp, out)
        }
    }
}

impl Encode for f32 {
    fn encode(&self, out: &mut impl io::Write) -> Result { (*self as f64).encode(out) }
}

impl Encode for f64 {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if self.is_nan() || self.is_infinite() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Serializing NaN and infinite values is not supported"));
        }

        out.write_u8(70)?;
        out.write_f64::<BigEndian>(*self)
    }
}

impl Encode for &str {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(109)?;
        out.write_u32::<BigEndian>(self.len() as u32)?;
        out.write_all(self.as_bytes())
    }
}

impl Encode for String {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(109)?;
        out.write_u32::<BigEndian>(self.len() as u32)?;
        out.write_all(self.as_bytes())
    }
}

impl Encode for &[u8] {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(109)?;
        out.write_u32::<BigEndian>(self.len() as u32)?;
        out.write_all(self)
    }
}

impl<K: Encode, V: Encode> Encode for BTreeMap<K, V> {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(116)?;
        out.write_u32::<BigEndian>(self.len() as u32)?;

        for (k, v) in self {
            k.encode(out)?;
            v.encode(out)?;
        }

        Ok(())
    }
}

impl<K: Encode, V: Encode> Encode for HashMap<K, V> {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        out.write_u8(116)?;
        out.write_u32::<BigEndian>(self.len() as u32)?;

        for (k, v) in self {
            k.encode(out)?;
            v.encode(out)?;
        }

        Ok(())
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, out: &mut impl io::Write) -> Result {
        if !self.is_empty() {
            out.write_u8(108)?;
            out.write_u32::<BigEndian>(self.len() as u32)?;

            for elem in self {
                elem.encode(out)?;
            }
        }

        out.write_u8(106)
    }
}

include!("tuple_encode.in.rs");
