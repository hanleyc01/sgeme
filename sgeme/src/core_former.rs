//! Core special forms

use crate::{eval::EvalError, primsyn::*};

pub enum Core {}

#[derive(Debug)]
pub enum CoreError {}

impl From<CoreError> for EvalError {
    fn from(value: CoreError) -> Self {
        Self::Simplify(value)
    }
}

pub type CoreFormError<T> = Result<T, CoreError>;

pub struct CoreFormer {}

impl CoreFormer {
    pub fn init() -> Self {
        Self {}
    }

    pub fn simplify(&self, stmst: &[Stmt]) -> CoreFormError<Core> {
        todo!()
    }
}
