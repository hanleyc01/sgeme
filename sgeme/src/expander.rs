//! Find and convert `syntax-rules` into pattern-based functions *in* Rust,
//! and then transform `Datum` into the new forms

use std::collections::HashMap;

use crate::datum::*;
use crate::primsyn::*;

/// Syntax expander, with accompanying primitive
/// syntax expansions for bootstrapping
pub struct Expander {}

#[derive(Debug)]
pub enum ExpanderError {
    IllegalNonatomic(String),
    IllegalNumberOfArgs(String),
    IdentifierExpected(String),
    ListExpected(String),
    CondElseExpected(String),
    IllegalContext(String),
    StringExpected(String),
    UnexpectedEof,
}

pub type ExpanderResult<T> = Result<T, ExpanderError>;

fn split_three<T>(ls: &[T]) -> Option<(&T, &T, &[T])> {
    if let Some((fs, tl)) = ls.split_first() {
        if let Some((snd, rst)) = tl.split_first() {
            Some((fs, snd, rst))
        } else {
            None
        }
    } else {
        None
    }
}

impl Expander {
    pub fn init() -> Self {
        Self {}
    }

    /// Finds the `define-syntax-rule` forms within `datum`, checking for some sort
    /// of correctness, and then removing them; making them into functions *on* `Datum` itself
    pub fn find_syntax_rules(&self, datum: &mut Datum) -> ExpanderResult<Vec<fn(&Datum) -> Datum>> {
        Ok(Vec::new())
    }

    fn expand_expr(&self, d: &Datum) -> ExpanderResult<Expr> {
        match d {
            Datum::Bool(b) => Ok(Expr::Bool(*b)),
            Datum::Fixnum(f) => Ok(Expr::Fixnum(*f)),
            Datum::Char(c) => Ok(Expr::Char(*c)),
            Datum::Vector(v) => Ok(Expr::Vector(v.clone())),
            Datum::Str(s) => Ok(Expr::Str(s.clone())),
            Datum::Eof => Err(ExpanderError::UnexpectedEof),
            Datum::Quote(abbrevprefix, datum) => match abbrevprefix {
                AbbrevPrefix::Comma => Ok(Expr::Unquote(*datum.clone())),
                AbbrevPrefix::Quote => Ok(Expr::Quote(*datum.clone())),
                _ => todo!(),
            },
            Datum::Symbol(s) => Ok(Expr::Symbol(s.clone())),
            Datum::List(ds) => match ds.split_first() {
                Some((head, tail)) => {
                 match head {
                    Datum::Symbol(s) => match s.as_ref() {
                        "define" => Err(ExpanderError::IllegalContext(
                            "can't define `define` there dummy".into(),
                        )),
                        "define-record" => {
                            Err(ExpanderError::IllegalContext("no records here".into()))
                        }
                        "import" => Err(ExpanderError::IllegalContext("can't import here".into())),
                        "export" => Err(ExpanderError::IllegalContext("can't export here".into())),
                        "quote" => Ok(Expr::Quote(Datum::List(tail.to_vec()))),
                        "lambda" => Ok(self.expand_lambda(tail)?),
                        "if" => Ok(self.expand_if(tail)?),
                        "cond" => Ok(self.expand_cond(tail)?),
                        "case" => Ok(self.expand_case(tail)?),
                        "and" => Ok(self.expand_and(tail)?),
                        "or" => Ok(self.expand_or(tail)?),
                        "when" => Ok(self.expand_when(tail)?),
                        "unless" => Ok(self.expand_unless(tail)?),
                        "let" => Ok(self.expand_let(tail)?),
                        "letrec" => Ok(self.expand_letrec(tail)?),
                        "begin" => Ok(self.expand_begin(tail)?),
                        _ => {
                            let rator = self.expand_expr(head)?;
                            let mut rand = Vec::new();
                            for datum in tail {
                                rand.push(self.expand_expr(datum)?)
                            }
                            Ok(Expr::ProcCall(Box::new(rator), rand))
                        }
                    },
                    _ => {
                        let rator = self.expand_expr(head)?;
                        let mut rand = Vec::new();
                        for datum in tail {
                            rand.push(self.expand_expr(datum)?)
                        }
                        Ok(Expr::ProcCall(Box::new(rator), rand))
                    }
                }
                },
                None => return Err(ExpanderError::IllegalNonatomic("()".to_string())),
            },
            _ => todo!(),
        }
    }

    /// Expand a datum of the form `(define ...)` into the relevant
    /// `primsyn::Def`, either of the form `Def::DefValue`, or `Def::DefFunc`
    fn expand_define(&self, ds: &[Datum]) -> ExpanderResult<Def> {
        if let Some((formals, body, rest)) = split_three(ds) {
            if !rest.is_empty() {
                return Err(ExpanderError::IllegalNumberOfArgs(
                    "u got too many define arguemnts".into(),
                ));
            }

            match formals {
                Datum::List(ls) => {
                    if ls.iter().all(|x| x.is_symbol()) {
                        let names: Vec<String> = ls.iter().map(|x| x.get_symbol_name()).collect();
                        if let Some((name, formals)) = names.split_first() {
                            Ok(Def::DefFunc(
                                name.to_owned(),
                                formals.into(),
                                self.expand_expr(body)?,
                            ))
                        } else {
                            return Err(ExpanderError::ListExpected(
                                "(define (<ident>+) ...) ; we need names".into(),
                            ));
                        }
                    } else {
                        Err(ExpanderError::IdentifierExpected(
                            "(define (<ident>+) <expr>) ; pls".into(),
                        ))
                    }
                }
                Datum::Symbol(name) => Ok(Def::DefValue(name.to_string(), self.expand_expr(body)?)),
                _ => Err(ExpanderError::IdentifierExpected(
                    "(define <ident> <expr>) OR (define (<ident>+) <expr>) ; pls".into(),
                )),
            }
        } else {
            Err(ExpanderError::IllegalNumberOfArgs(
                "(define ...) broh u need more than just `define`".into(),
            ))
        }
    }

    /// Expand a datum of the form `(define-record ...)` into a
    /// `primsyn::Def::DefRecord`
    fn expand_define_record(&self, ds: &[Datum]) -> ExpanderResult<Def> {
        match ds.split_first() {
            // (define-record <ident> (<ident> ...))
            Some((head, tail)) => {
                if head.is_symbol() {
                    match tail.split_first() {
                        Some((mems, nothing)) => {
                            if nothing.is_empty() {
                                match mems {
                                    Datum::List(ms) => {
                                        if ms.iter().all(|x| x.is_symbol()) {
                                            let name = head.get_symbol_name();
                                            let members =
                                                ms.iter().map(|x| x.get_symbol_name()).collect();
                                            Ok(Def::DefRecord(name, members))
                                        } else {
                                            Err(ExpanderError::IdentifierExpected(
                                                "(define-record <ident> (<ident>*)".to_owned(),
                                            ))
                                        }
                                    }
                                    _ => Err(ExpanderError::ListExpected(
                                        "(define-record <ident> (<ident>)*)".to_owned(),
                                    )),
                                }
                            } else {
                                Err(ExpanderError::IllegalNumberOfArgs(
                                    "(define-record <ident> (<ident>*) ; nothing else!!".to_owned(),
                                ))
                            }
                        }
                        None => Err(ExpanderError::IllegalNumberOfArgs(
                            "(define-record <ident> (<ident>*)".to_owned(),
                        )),
                    }
                } else {
                    Err(ExpanderError::IdentifierExpected(
                        "(define-record <ident> ...)".to_owned(),
                    ))
                }
            }
            None => Err(ExpanderError::IllegalNumberOfArgs(
                "(define-record <ident> (<ident>*)".to_owned(),
            )),
        }
    }

    /// Expand a datum of the form `(import ...)` to `primsyn::Import::Import`
    fn expand_import(&self, ds: &[Datum]) -> ExpanderResult<Import> {
        // (import <string>+)
        if ds.iter().all(|x| x.is_string()) && !ds.is_empty() {
            Ok(Import::Import(ds.iter().map(|x| x.get_string()).collect()))
        } else {
            Err(ExpanderError::StringExpected(
                "(import <string>+) ; this is how u do imports".into(),
            ))
        }
    }

    /// Expand a datum of the form `(export ...)` to `primsyn::Import::Export`
    fn expand_export(&self, ds: &[Datum]) -> ExpanderResult<Import> {
        // (export <ident>+)
        if ds.iter().all(|x| x.is_symbol()) && !ds.is_empty() {
            Ok(Import::Export(
                ds.iter().map(|x| x.get_symbol_name()).collect(),
            ))
        } else {
            Err(ExpanderError::StringExpected(
                "(export <ident>+) ; this is how u do exports".into(),
            ))
        }
    }

    /// Expand a datum of the form `(lambda (...) ...)` to `primsyn::Expr::Lambda`
    fn expand_lambda(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        match ds.split_first() {
            Some((head, tail)) => match head {
                Datum::List(fs) => {
                    if !fs.iter().all(|x| x.is_symbol()) {
                        return Err(ExpanderError::IdentifierExpected(
                            "(lambda (<ident>*) <expr>)".into(),
                        ));
                    }

                    match tail.split_first() {
                        Some((x, xs)) => {
                            if !xs.is_empty() {
                                return Err(ExpanderError::IllegalNumberOfArgs(
                                    "(lambda (<ident>*) <expr>)".into(),
                                ));
                            }

                            let formals = fs.iter().map(|x| x.get_symbol_name()).collect();
                            let body = self.expand_expr(x)?;

                            Ok(Expr::Lambda(formals, Box::new(body)))
                        }
                        None => Err(ExpanderError::IllegalNumberOfArgs(
                            "(lambda (<ident>*) <expr>)".into(),
                        )),
                    }
                }
                _ => Err(ExpanderError::ListExpected(
                    "(lambda (<ident>*) <expr>)".into(),
                )),
            },
            None => Err(ExpanderError::IllegalNumberOfArgs(
                "(lambda (<ident>*) <expr>)".into(),
            )),
        }
    }

    /// Expand a datum of the form `(if <expr> <expr> <expr>)` to `primsyn::Expr::If`
    fn expand_if(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        match ds.split_first() {
            Some((condition, tail)) => match tail.split_first() {
                Some((r#then, tail)) => match tail.split_first() {
                    Some((r#else, other)) => {
                        if !other.is_empty() {
                            return Err(ExpanderError::IllegalNumberOfArgs(
                                "(if <expr> <expr> <expr>)".into(),
                            ));
                        }

                        let condition_expr = self.expand_expr(condition)?;
                        let then_expr = self.expand_expr(r#then)?;
                        let else_expr = self.expand_expr(r#else)?;

                        Ok(Expr::If(
                            Box::new(condition_expr),
                            Box::new(then_expr),
                            Box::new(else_expr),
                        ))
                    }
                    None => Err(ExpanderError::IllegalNumberOfArgs(
                        "(if <expr> <expr> <expr>)".into(),
                    )),
                },
                None => Err(ExpanderError::IllegalNumberOfArgs(
                    "(if <expr> <expr> <expr>)".into(),
                )),
            },
            None => Err(ExpanderError::IllegalNumberOfArgs(
                "(if <expr> <expr> <expr>)".into(),
            )),
        }
    }

    /// Expand some `ds: &[Datum]` of the form `((<expr> <expr>) ...)` into acceptable tuples of
    /// expressions
    fn expand_cond_branches(&self, ds: &[Datum]) -> ExpanderResult<Vec<(Expr, Expr)>> {
        let mut branches = Vec::new();
        for datum in ds {
            if let Datum::List(bs) = datum {
                if bs.len() == 2 {
                    let car = bs[0].clone();
                    let cdr = bs[1].clone();
                    let car_expr = self.expand_expr(&car)?;
                    let cdr_expr = self.expand_expr(&cdr)?;

                    branches.push((car_expr, cdr_expr));
                } else {
                    return Err(ExpanderError::ListExpected(
                        "cond branches are of the form (<expr> <expr>)".into(),
                    ));
                }
            } else {
                return Err(ExpanderError::ListExpected(
                    "(cond (<expr> <expr> ...)) ; need lists!!".into(),
                ));
            }
        }
        Ok(branches)
    }

    fn expand_cond_else(&self, d: &Datum) -> ExpanderResult<Expr> {
        if let Datum::List(ls) = d {
            let car = ls[0].clone();
            let cdr = ls[1].clone();
            if let Datum::Symbol(s) = car {
                if s == "else" {
                    let cdr_expr = self.expand_expr(&cdr)?;
                    Ok(cdr_expr)
                } else {
                    Err(ExpanderError::CondElseExpected(
                        "(else <expr>) ; else expected in final cond branch".into(),
                    ))
                }
            } else {
                Err(ExpanderError::CondElseExpected(
                    "(else <expr>) ; else expected in final cond branch".into(),
                ))
            }
        } else {
            Err(ExpanderError::ListExpected(
                "(else <expr>) ; final else clause of cond expected".into(),
            ))
        }
    }

    /// Expand a datum of the form `(cond (...) (...) ...)` to `primsyn::Expr::Cond`
    fn expand_cond(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        if let Some((last, init)) = ds.split_last() {
            let branches = self.expand_cond_branches(init)?;
            let last_expr = self.expand_cond_else(last)?;
            Ok(Expr::Cond(branches, Box::new(last_expr)))
        } else {
            Err(ExpanderError::IllegalNumberOfArgs(
                "(cond (<expr> <expr>) ... (else <expr>))".into(),
            ))
        }
    }

    /// Expand a datum of the form `(<datum>* <expr>)`
    fn expand_case_branches(&self, ds: &[Datum]) -> ExpanderResult<Vec<(Vec<Datum>, Sequence)>> {
        let mut branches = Vec::new();
        for datum in ds {
            if let Datum::List(ls) = datum {
                if let Some((head, tail)) = ls.split_first() {
                    if let Datum::List(data) = head {
                        let mut exprs = Vec::new();
                        for d in data {
                            exprs.push(self.expand_expr(d)?)
                        }
                        branches.push((data.clone(), exprs))
                    } else {
                        return Err(ExpanderError::ListExpected(
                            "(<datum>* <sequence>) ; list expected in case branch".into(),
                        ));
                    }
                }
            } else {
                return Err(ExpanderError::ListExpected(
                    "(<datum>* <sequence>) ; list expected in case branch".into(),
                ));
            }
        }
        Ok(branches)
    }

    fn expand_case_else(&self, d: &Datum) -> ExpanderResult<Sequence> {
        if let Datum::List(ds) = d {
            let mut exprs = Vec::new();
            for datum in ds {
                exprs.push(self.expand_expr(datum)?);
            }
            Ok(exprs)
        } else {
            Err(ExpanderError::ListExpected(
                "(else <sequence>) ; list expected as else case clause".into(),
            ))
        }
    }

    /// Expand a datum of the form `(case <expr> (...))` to `primsyn::Expr::Case`
    fn expand_case(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        if let Some((analysand, branches)) = ds.split_first() {
            if let Some((last, init)) = branches.split_last() {
                let analysand_expr = self.expand_expr(analysand)?;
                let branches = self.expand_case_branches(branches)?;
                let r#else = self.expand_case_else(last)?;
                Ok(Expr::Case(Box::new(analysand_expr), branches, r#else))
            } else {
                Err(ExpanderError::IllegalNumberOfArgs(
                    "case <expr> (<datum>* <sequence>) ... (else <sequence>) ; missing branches and else clause".into()
                ))
            }
        } else {
            Err(ExpanderError::IllegalNumberOfArgs(
                "(case <expr> (<datum>* <sequence>) ... (else <sequence>)) ; invalid number of arguments".into()
            ))
        }
    }

    fn expand_helper(&self, ds: &[Datum], f: fn(Vec<Expr>) -> Expr) -> ExpanderResult<Expr> {
        let mut exprs = Vec::new();
        for datum in ds {
            exprs.push(self.expand_expr(datum)?)
        }
        Ok(f(exprs))
    }

    /// Expand a datum of the form `(and ...)` to `primsyn::Expr::And`
    fn expand_and(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        self.expand_helper(ds, |xs| Expr::And(xs))
    }

    /// Expand a datum of the form `(or ...)` to `primsyn::Expr::Or`
    fn expand_or(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        self.expand_helper(ds, |xs| Expr::Or(xs))
    }

    fn when_unless_helper(
        &self,
        ds: &[Datum],
        f: fn((Expr, Vec<Expr>)) -> Expr,
    ) -> ExpanderResult<Expr> {
        if let Some((hd, tl)) = ds.split_first() {
            let condition = self.expand_expr(hd)?;
            let mut seq = Vec::new();
            for datum in tl {
                seq.push(self.expand_expr(datum)?)
            }
            Ok(f((condition, seq)))
        } else {
            Err(ExpanderError::IllegalNumberOfArgs("This label is the target of a goto from outside of the block containing this label AND this block has an automatic variable with an initializer AND your window wasn't wide enough to read this whole error message".into()))
        }
    }

    /// Expand a datum of the form `(when <expr> <expr>)` to `primsyn::Expr::When`
    fn expand_when(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        self.when_unless_helper(ds, |(x, ys)| Expr::When(Box::new(x), ys))
    }

    /// Expand a datum of the form `(unless <expr> <expr>)` to `primsyn::Expr::Unless`
    fn expand_unless(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        self.when_unless_helper(ds, |(x, ys)| Expr::Unless(Box::new(x), ys))
    }

    fn expand_let_branches(&self, ds: &[Datum]) -> ExpanderResult<Vec<(String, Expr)>> {
        let mut branches = Vec::new();
        for datum in ds {
            if let Datum::List(ls) = datum {
                if let Some((name, body, rst)) = split_three(ls) {
                    if !rst.is_empty() {
                        return Err(ExpanderError::IllegalNumberOfArgs(
                            "<let> and <letrec> forms need to assign things lmao get it right"
                                .into(),
                        ));
                    }

                    if let Datum::Symbol(n) = name {
                        let nm = n.to_string();
                        let body_expr = self.expand_expr(body)?;
                        branches.push((nm, body_expr))
                    } else {
                        return Err(ExpanderError::ListExpected(
                            "<let> and <letrec> forms need to assign things lmao get it right"
                                .into(),
                        ));
                    }
                } else {
                    return Err(ExpanderError::ListExpected(
                        "<let> and <letrec> forms need to assign things lmao get it right".into(),
                    ));
                }
            } else {
                return Err(ExpanderError::ListExpected(
                    "<let> and <letrec> forms need to assign things lmao get it right".into(),
                ));
            }
        }
        Ok(branches)
    }

    /// Expand a datum of the form `(let ((ident expr) ...) expr)` to `primsyn::Expr::Let`
    fn expand_let(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        if let Some((body, branches)) = ds.split_last() {
            let branchaises = self.expand_let_branches(branches)?;
            let body_expr = self.expand_expr(body)?;
            Ok(Expr::Let(branchaises, Box::new(body_expr)))
        } else {
            Err(ExpanderError::IllegalNumberOfArgs(
                "(let ((<ident> <expr>) ...) <expr>) ; reminder of `let` form bruv".into(),
            ))
        }
    }

    /// Expand a datum of the form `(letrec ((ident expr) ...) expr)` to `primsyn::Expr::LetRec`
    fn expand_letrec(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        if let Some((body, branches)) = ds.split_last() {
            let branchaises = self.expand_let_branches(branches)?;
            let body_expr = self.expand_expr(body)?;
            Ok(Expr::Let(branchaises, Box::new(body_expr)))
        } else {
            Err(ExpanderError::IllegalNumberOfArgs(
                "(letrec ((<ident> <expr>) ...) <expr>) ; reminder of `let` form bruv".into(),
            ))
        }
    }

    /// Expand a datum of the form `(begin expr ...)` to `primsyn::Expr::Begin`
    fn expand_begin(&self, ds: &[Datum]) -> ExpanderResult<Expr> {
        let mut exprs = Vec::new();
        for datum in ds {
            exprs.push(self.expand_expr(datum)?)
        }
        Ok(Expr::Begin(exprs))
    }

    fn expand_datum(&self, d: &Datum, prgrm: &mut Program) -> ExpanderResult<()> {
        match d {
            Datum::List(ds) => match ds.split_first() {
                Some((head, tail)) => match head {
                    Datum::Symbol(s) => match s.as_ref() {
                        "define" => prgrm.stmts.push(Stmt::Def(self.expand_define(tail)?)),
                        "define-record" => prgrm
                            .stmts
                            .push(Stmt::Def(self.expand_define_record(tail)?)),
                        "import" => prgrm.imports.push(self.expand_import(tail)?),
                        "export" => prgrm.imports.push(self.expand_export(tail)?),
                        "quote" => prgrm
                            .stmts
                            .push(Stmt::Expr(Expr::Quote(Datum::List(tail.to_vec())))),
                        "lambda" => prgrm.stmts.push(Stmt::Expr(self.expand_lambda(tail)?)),
                        "if" => prgrm.stmts.push(Stmt::Expr(self.expand_if(tail)?)),
                        "cond" => prgrm.stmts.push(Stmt::Expr(self.expand_cond(tail)?)),
                        "case" => prgrm.stmts.push(Stmt::Expr(self.expand_case(tail)?)),
                        "and" => prgrm.stmts.push(Stmt::Expr(self.expand_and(tail)?)),
                        "or" => prgrm.stmts.push(Stmt::Expr(self.expand_or(tail)?)),
                        "when" => prgrm.stmts.push(Stmt::Expr(self.expand_when(tail)?)),
                        "unless" => prgrm.stmts.push(Stmt::Expr(self.expand_unless(tail)?)),
                        "let" => prgrm.stmts.push(Stmt::Expr(self.expand_let(tail)?)),
                        "letrec" => prgrm.stmts.push(Stmt::Expr(self.expand_letrec(tail)?)),
                        "begin" => prgrm.stmts.push(Stmt::Expr(self.expand_begin(tail)?)),
                        _ => prgrm.stmts.push(Stmt::Expr(self.expand_expr(d)?)),
                    },
                    _ => prgrm.stmts.push(Stmt::Expr(self.expand_expr(d)?)),
                },
                None => return Err(ExpanderError::IllegalNonatomic("()".to_string())),
            },
            _ => prgrm.stmts.push(Stmt::Expr(self.expand_expr(d)?)),
        }
        Ok(())
    }

    /// Given some syntax expanders, transform the `Datum` into new datum
    pub fn expand_prgrm(&self, src: &Datum) -> ExpanderResult<Program> {
        let mut prgrm = Program::init();
        match src {
            Datum::List(vs) => {
                for datum in vs {
                    self.expand_datum(&datum, &mut prgrm)?;
                }
                Ok(prgrm)
            }
            _ => {
                self.expand_datum(src, &mut prgrm)?;
                Ok(prgrm)
            }
        }
    }
}
