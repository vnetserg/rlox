use rlox::Lox;

use structopt::StructOpt;
use sysexit::Code;

use std::{fs, io, path::PathBuf, process::exit};

#[derive(StructOpt)]
struct Opt {
    /// Script path (if not present, run in interactive mode)
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,
}

fn run_file(mut lox: Lox, path: PathBuf) {
    let file = fs::OpenOptions::new()
        .read(true)
        .open(&path)
        .unwrap_or_else(|err| {
            eprintln!("Failed to open '{:?}': {}", path, err);
            exit(Code::NoInput as i32);
        });
    let reader = io::BufReader::new(file);
    let result = lox.run(reader, io::stdout());
    if let Err(err) = result {
        eprintln!("{}", err);
        exit(Code::DataErr as i32);
    }
}

fn run_interactive(mut lox: Lox) {
    let result = lox.run_interactive(io::stdin().lock(), io::stdout(), io::stderr());
    if let Err(err) = result {
        eprintln!("{}", err);
        exit(Code::IoErr as i32);
    }
}

fn main() {
    let opt = Opt::from_args();
    let lox = Lox::default();
    if let Some(path) = opt.path {
        run_file(lox, path);
    } else {
        run_interactive(lox);
    }
}
