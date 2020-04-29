use std::fmt;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::Chars;

use structopt::StructOpt;

use silly_lex::*;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    input_file: PathBuf,
}

fn main() {
    let args = Args::from_args();
    let regex = read_to_string(args.input_file).unwrap();

    for token in Lexer::new(&regex).iter() {
        println!("{}", token);
    }
}
