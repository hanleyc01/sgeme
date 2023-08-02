#![allow(unused, dead_code)]

use std::error::Error;
use std::fs;

mod ast;
mod datum;
mod parser;
mod read;
mod token;

use crate::token::{Logos, Token};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file_contents = fs::read_to_string("./sgeme.scm")?;
    println!("{file_contents}");

    let mut lexer = Token::lexer(&file_contents);

    for token in lexer {
        println!("{token:?}");
    }
    Ok(())
}
