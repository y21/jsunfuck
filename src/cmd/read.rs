use std::io;
use std::io::Read;

pub fn read() -> anyhow::Result<()> {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;

    super::process(&source);

    Ok(())
}
