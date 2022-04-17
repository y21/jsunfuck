use anyhow::Context;
use clap::ArgMatches;

pub fn run(args: &ArgMatches) -> anyhow::Result<()> {
    let path = args.value_of("file").context("Missing file")?;
    let source = std::fs::read_to_string(path)?;

    super::process(&source);

    Ok(())
}
