use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut encode_file = File::create("src/tuple_encode.in.rs").unwrap();
    // let mut decode_file = File::create("src/tuple_decode.in.rs").unwrap();

    for i in 1..=32 {
        write!(&mut encode_file, "impl<")?;
        write_types(&mut encode_file, i, ": Encode")?;
        write!(&mut encode_file, "> Encode for (")?;
        write_types(&mut encode_file, i, "")?;
        writeln!(&mut encode_file, ") {{")?;
        writeln!(&mut encode_file, "    fn encode(&self, out: &mut impl io::Write) -> Result {{")?;
        writeln!(&mut encode_file, "        out.write_u8(104)?;")?;
        writeln!(&mut encode_file, "        out.write_u8({})?;", i)?;

        for j in 0..i {
            writeln!(&mut encode_file, "        self.{}.encode(out)?;", j)?;
        }

        writeln!(&mut encode_file, "        Ok(())\n    }}\n}}")?;
        writeln!(&mut encode_file, "")?;

        // write!(&mut decode_file, "impl<")?;
        // write_types(&mut decode_file, i, "")?;
        // write!(&mut decode_file, "> TryInto<(")?;
        // write_types(&mut decode_file, i, "")?;
        // write!(&mut decode_file, ")> for Type where ")?;
        // write_types(&mut decode_file, i, ": TryFrom<Type, Error=TryFromErnieError>")?;
        // writeln!(&mut decode_file, " {{")?;
        // writeln!(&mut decode_file, "    type Error = TryFromErnieError;")?;
        // writeln!(&mut decode_file, "")?;
        // write!(&mut decode_file, "    fn try_into(self) -> Result<(")?;
        // write_types(&mut decode_file, i, "")?;
        // writeln!(&mut decode_file, "), Self::Error> {{")?;
        // writeln!(&mut decode_file, "        match self {{")?;
        // writeln!(&mut decode_file, "           Type::Tuple(data) => {{")?;
        // writeln!(&mut decode_file, "               if data.len() != {} {{", i)?;
        // writeln!(&mut decode_file, "                   return Err(TryFromErnieError::InvalidTupleArity(data.len()))")?;
        // writeln!(&mut decode_file, "               }}")?;
        // writeln!(&mut decode_file, "               let mut iter = data.into_iter();")?;
        // write!(&mut decode_file,   "               Ok((")?;
        // write_types(&mut decode_file, i, "::try_from(iter.next().unwrap())?")?;
        // writeln!(&mut decode_file, "))")?;
        // writeln!(&mut decode_file, "            }}")?;
        // writeln!(&mut decode_file, "            _ => Err(TryFromErnieError::InvalidType(self))")?;
        // writeln!(&mut decode_file, "        }}")?;
        // writeln!(&mut decode_file, "    }}")?;
        // writeln!(&mut decode_file, "}}")?;
        // writeln!(&mut decode_file, "")?;
    }

    Ok(())
}

fn write_types(file: &mut impl Write, count: u32, suffix: &str) -> io::Result<()> {
    for i in 0..count {
        write!(file, "T{}{}, ", i, suffix)?;
    }

    Ok(())
}
