//! Runtime representations of values within Scheme

use std::collections::HashMap;

use crate::datum::Datum;

pub type Env = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    Char(char),
    Bool(bool),
    Str(String),
    Vector(Vec<Self>),
    Fixnum(i32),
    Quote(Datum),
    Env(Env),
    Closure(Box<Self>, Env),
}
