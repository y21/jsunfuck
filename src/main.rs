use anyhow::bail;
use clap::Arg;
use clap::Command;

mod cmd;
mod deduce;
mod serialize;

fn main() -> anyhow::Result<()> {
    let app = Command::new("jsunfuck")
        .about("Reverse jsfuck.com output")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .override_help("Process a JavaScript file")
                .arg(Arg::new("file").required(true)),
        )
        .subcommand(Command::new("read").override_help("Read JavaScript code from stdin"));

    let matches = app.get_matches();
    match matches.subcommand() {
        Some(("run", args)) => cmd::run(args),
        Some(("read", _)) => cmd::read(),
        _ => bail!("Unimplemented"),
    }
}
