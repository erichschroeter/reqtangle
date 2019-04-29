use std::error::Error;
use std::env;
use std::path::Path;
use docopt::Docopt;
use serde::Deserialize;

mod gather;

const USAGE: &'static str = "
reqtangle

Usage:
  reqtangle gather [<paths>...]
  reqtangle check [<paths>...]
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_gather: bool,
    cmd_check: bool,
    arg_paths: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.cmd_gather {
        if args.arg_paths.is_empty() {
            let current_dir = env::current_dir()?;
            gather::traverse(&current_dir)?;
        } else {
            for path in args.arg_paths {
                let path = Path::new(&path);
                gather::traverse(&path)?;
            }
        }
    } else if args.cmd_check {
        unimplemented!();
    }
    Ok(())
}
