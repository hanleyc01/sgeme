use crate::ast::{Def, Export, Expr, Import, Literal, ParsedModule, Stmt};
use crate::datum::{AbbrevPrefix, Datum, SimpleDatum};
use crate::token::Token;

pub struct Reader {
}

pub enum ReadError {
}

pub type ReadResult<T> = Result<T, ReadError>;

impl Reader {
    pub fn init() -> Self {
        Self {}
    }

    pub fn read(&self, src: &str) -> ReadResult<Datum> {
        todo!()
    }
}
