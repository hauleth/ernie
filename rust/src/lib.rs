use std::io;

mod encode;
mod decode;

pub use encode::Encode;
pub use decode::Type;

pub fn encode(data: impl Encode, out: &mut impl io::Write) -> io::Result<()> {
    out.write_all(&[131]).and_then(|_| data.encode(out))
}

pub fn decode(reader: &mut impl io::Read) -> io::Result<Type> {
    let mut header = [0u8; 1];

    reader.read_exact(&mut header)?;

    if header[0] == 131 {
        decode::decode_type(reader)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic byte"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
