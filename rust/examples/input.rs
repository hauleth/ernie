use std::fs;
use std::collections::BTreeMap;
use std::convert::TryInto;

// type Output = ;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::open("file.ernie")?;
    let value = ernie::decode(&mut file).unwrap();
    println!("{:#?}", value);

    // let t: (Vec<ernie::Type>, String, i32, f64, BTreeMap<ernie::Type, ernie::Type>) = value.try_into().unwrap();
    // let mut t: Vec<ernie::Type> = value.try_into().unwrap();
    let s: BTreeMap<ernie::Type, ernie::Type> = value.try_into().unwrap();

    println!("{:#?}", s);

    Ok(())
}
