use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::create("file.ernie")?;
    let mut map = HashMap::new();
    let _ = map.insert(1, "foo");
    let _ = map.insert(2, "bar");

    // let value = (vec![1,2,3,4,5], "Zażółć gęślą jaźń", 10, 3.14, map);
    let value = map;

    println!("{:#?}", value);
    ernie::encode(value, &mut file)?;

    Ok(())
}
