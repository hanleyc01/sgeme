//! Convert `Datum` into `Value`

use crate::core_former::{CoreError, CoreFormer};
use crate::datum::Datum;
use crate::primsyn::*;

use rs_mir::MIRContext;

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug)]
pub enum EvalError {
    UnboundVariable(String),
    Simplify(CoreError),
}

pub struct Evaluator<'a> {
    ctx: &'a mut MIRContext,
}

impl<'a> Evaluator<'a> {
    pub fn init(ctx: &'a mut MIRContext) -> Self {
        Self { ctx }
    }

    /// Take some `primsyn::Program`, and evaluate using the `rs-mir` crate
    pub fn compile_program(&mut self, prgrm: &Program) -> EvalResult<()> {
        // TODO: figure out imports!
        let _imports: &[Import] = &prgrm.imports;
        let stmts = &prgrm.stmts;

        let core_former = CoreFormer::init();
        let core_stmts = core_former.simplify(stmts)?;

        Ok(())
    }
}
