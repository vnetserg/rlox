use rlox::{Lox, errors::*};

use structopt::StructOpt;
use sysexit::Code;

use std::{fs, io::{self, Write}, path::PathBuf, process::exit};

#[derive(StructOpt)]
struct Opt {
    /// Script path (if not present, run in interactive mode)
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>
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
    let result = lox.execute(reader, io::stdout());
    if let Err(err) = result {
        eprintln!("{}", err.display_chain());
    }
}

fn run_interactive(mut lox: Lox) {
    let mut line = String::new();
    loop {
        print!("> ");
        if let Err(err) = io::stdout().flush() {
            eprintln!("\nError flushing stdout: {}", err);
            exit(Code::IoErr as i32);
        }

        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    println!();
                    break;
                }
            },
            Err(err) => {
                eprintln!("\nFailed to read line: {}", err);
                exit(Code::IoErr as i32);
            }
        }

        let result = lox.execute(&mut line.as_bytes(), io::stdout());
        if let Err(err) = result {
            eprintln!("{}", err.display_chain());
        }
        line.clear();
    }
}

fn main() {
    let opt = Opt::from_args();
    let lox = Lox::new();
    if let Some(path) = opt.path {
        run_file(lox, path);
    } else {
        run_interactive(lox);
    }
}
