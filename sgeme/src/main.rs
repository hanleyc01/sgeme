#![allow(unused, dead_code)]

use std::error::Error;
use std::fs;

mod core_former;
mod datum;
mod eval;
mod expander;
mod primsyn;
mod read;
mod token;

use expander::Expander;
use read::Reader;
use token::{Logos, Token};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file_contents = fs::read_to_string("./test-src/sgeme.ss")?;
    let tokens = Token::really_lex(file_contents);
    let mut iter = tokens.iter().peekable();
    let mut reader = Reader::init(false, true, &mut iter);
    let res = reader.read();
    match res {
        Ok(r) => {
            let expander: Expander = Expander::init();
            let other_res = expander.expand_prgrm(&r);
            dbg!(other_res);
        }
        Err(e) => {
            dbg!(e);
        }
    };
    Ok(())
}
