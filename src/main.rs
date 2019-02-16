pub mod syntax;

use std::env;
use std::fs;
use std::io::{self, prelude::*};

fn display_usage() {
    let usage = "zymogen";
}

fn main() -> io::Result<()> {
    let mut inputs = Vec::new();
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "--help" | "-h" => display_usage(),
            _ => match fs::read_to_string(&arg) {
                Ok(data) => inputs.push(data),
                Err(e) => panic!("Error encountered while trying to read {}: {}", arg, e),
            },
        }
    }
    println!("zymogen interpreter");
    for s in inputs {
        println!("{}", s);
    }

    println!("REPL mode:");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        let mut indent: u32 = 0;
        let mut depth: i32 = 0;
        loop {
            handle.read_line(&mut buffer)?;
            let left = buffer.matches('(').count() as i32;
            let right = buffer.matches(')').count() as i32;
            if left == right {
                break;
            }

            // Do some pretty formatting for indent levels
            let delta = left - right;
            if delta > depth {
                indent += 4;
                depth = delta;
            } else if delta < depth {
                indent -= 4;
                depth = delta;
            }
            print!("{}", (0..indent).map(|_| ' ').collect::<String>());
            io::stdout().flush()?;
        }
        let tokens = match syntax::parse(&buffer.trim()) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprint!("{}", e);
                break;
            }
        };

        println!("===> {:#?}", tokens);
        buffer.clear();
    }
    Ok(())
}
