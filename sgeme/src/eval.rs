//! Convert `Datum` into `Value`

use crate::datum::Datum;
use crate::value::{self, Env, Value};

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug)]
pub enum EvalError {
    UnboundVariable(String),
}

pub struct Evaluator {}

impl Evaluator {
    pub fn init() -> Self {
        Self {}
    }

    pub fn eval(&self, data: &Datum, env: &mut Env) -> EvalResult<Value> {
        match data {
            Datum::Bool(b) => Ok(Value::Bool(*b)),
            Datum::Char(c) => Ok(Value::Char(*c)),
            Datum::Fixnum(f) => Ok(Value::Fixnum(*f)),
            Datum::Vector(fs) => {
                let mut vs = Vec::new();
                for f in fs {
                    vs.push(self.eval(f, env)?);
                }

                Ok(Value::Vector(vs))
            }
            Datum::Str(s) => Ok(Value::Str(s.to_owned())),
            Datum::Symbol(s) => match env.get(s) {
                None => Err(EvalError::UnboundVariable(s.to_owned())),
                Some(v) => Ok(v.clone()),
            },
            Datum::List(ls) => {
                todo!()
            }
            _ => todo!(),
        }
    }
}
