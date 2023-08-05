//! Primitive, or "Core" special forms, which have meaning by the gracious
//! grant of me (the compiler)

use crate::datum::Datum;

#[derive(Debug)]
pub struct Program {
    pub imports: Vec<Import>,
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn init() -> Self {
        Self {
            imports: Vec::new(),
            stmts: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Import {
    Export(Vec<String>),
    Import(Vec<String>),
}

#[derive(Debug)]
pub enum Stmt {
    Def(Def),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Def {
    DefValue(String, Expr),
    DefFunc(String, Vec<String>, Expr),
    DefRecord(String, Vec<String>),
}

#[derive(Debug)]
pub enum Expr {
    Symbol(String),
    Bool(bool),
    Fixnum(i32),
    Vector(Vec<Datum>),
    Char(char),
    Str(String),
    Quote(Datum),
    Unquote(Datum),
    ProcCall(Box<Self>, Vec<Self>),
    Lambda(Vec<String>, Box<Self>),
    If(Box<Self>, Box<Self>, Box<Self>),
    Cond(Vec<(Self, Self)>, Box<Self>),
    Case(Box<Self>, Vec<(Vec<Datum>, Sequence)>, Sequence),
    And(Vec<Self>),
    Or(Vec<Self>),
    When(Box<Self>, Sequence),
    Unless(Box<Self>, Sequence),
    Let(Vec<BindingSpec>, Box<Self>),
    LetRec(Vec<BindingSpec>, Box<Self>),
    Begin(Sequence),
}

pub type BindingSpec = (String, Expr);

pub type Sequence = Vec<Expr>;
