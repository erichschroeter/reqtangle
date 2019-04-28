use std::fs;
use docopt::Docopt;
use serde::Deserialize;

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
    arg_paths: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.cmd_gather {
        println!("req gather {}", args.arg_paths.unwrap_or_default());
        unimplemented!("req gather")
    } else if args.cmd_check {
        println!("req check {}", args.arg_paths.unwrap_or_default());
    }
}
