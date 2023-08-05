//! Bootsrapping reader with some associated options,
//! not fully R7RS compliant

use std::iter::Peekable;
use std::slice::Iter;

use crate::datum::{AbbrevPrefix, Datum};
use crate::expander::{Expander, ExpanderError};
use crate::token::{Logos, Token};

/// Reader struct which contains reading options:
/// + `case_insensitive`: treat all symbols read as `lowercase`
/// + `bracket_paren`: treat all brackets as being parentheses
pub struct Reader<'a> {
    pub case_insensitive: bool,
    pub bracket_paren: bool,
    src: &'a mut Peekable<Iter<'a, Token>>,
}

#[derive(Debug)]
pub enum ReadError {
    UnknownSymbol(Token),
    UnexpectedListTerminator(Token),
    ExpectedListTerminator(Token),
    UnhandledQuote,
    ExpandError(ExpanderError),
    Nothing,
}

impl From<ExpanderError> for ReadError {
    fn from(value: ExpanderError) -> Self {
        Self::ExpandError(value)
    }
}

pub type ReadResult<T> = Result<T, ReadError>;

impl<'a> Reader<'a> {
    pub fn init(
        case_insensitive: bool,
        bracket_paren: bool,
        src: &'a mut Peekable<Iter<'a, Token>>,
    ) -> Self {
        Self {
            case_insensitive,
            bracket_paren,
            src: src,
        }
    }

    fn read_sexpr(&mut self, separator: Token) -> ReadResult<Datum> {
        let terminator = if separator == Token::LParen {
            Token::RParen
        } else if (separator == Token::LBracket) && self.bracket_paren {
            Token::RBracket
        } else {
            return Err(ReadError::UnknownSymbol(separator));
        };

        let mut sexpr = Vec::new();

        let curr = self.src.peek().unwrap();
        match curr {
            Token::Dot => return Ok(Datum::List(sexpr)),
            _ => (),
        }

        let tmp = self.read_expr()?;
        sexpr.push(tmp);

        // check if this is a dotted pair
        match self.src.peek().unwrap() {
            Token::Dot => {
                self.src.next();
                let other = self.read_expr()?;
                let maybe_term = self.src.next().unwrap();
                dbg!("here");
                if maybe_term == &terminator {
                    return Ok(Datum::DottedList(sexpr, Box::new(other)));
                } else {
                    return Err(ReadError::ExpectedListTerminator(maybe_term.clone()));
                }
            }
            _ => (),
        }

        // otherwise, continue
        loop {
            if self.src.peek().unwrap().clone() == &terminator {
                self.src.next();
                return Ok(Datum::List(sexpr));
            // in the same way as above, check if there is a dot, and then exit!
            } else if self.src.peek().unwrap().clone() == &Token::Dot {
                self.src.next();
                let other = self.read_expr()?;
                let maybe_term = self.src.next().unwrap();
                if matches!(maybe_term, terminator) {
                    return Ok(Datum::DottedList(sexpr, Box::new(other)));
                } else {
                    return Err(ReadError::ExpectedListTerminator(maybe_term.clone()));
                }
            } else {
                let next = self.read_expr()?;
                sexpr.push(next)
            }
        }
    }

    fn read_quote(&mut self, quote_tok: Token) -> ReadResult<Datum> {
        let prefix = match quote_tok {
            Token::Quote => AbbrevPrefix::Quote,
            Token::Quasi => AbbrevPrefix::Quasi,
            Token::Comma => AbbrevPrefix::Comma,
            Token::CommaAt => AbbrevPrefix::CommaAt,
            _ => unreachable!("Unexpected {:?} in this function", quote_tok),
        };
        let expr = self.read_expr()?;
        match expr {
            Datum::Eof => Err(ReadError::UnhandledQuote),
            _ => Ok(Datum::Quote(prefix, Box::new(expr))),
        }
    }

    fn read_bytevector(&mut self) -> ReadResult<Datum> {
        todo!("implement bytevectors?")
    }

    fn read_vector(&mut self, separator: Token) -> ReadResult<Datum> {
        if let Datum::List(vs) = self.read_sexpr(separator)? {
            Ok(Datum::Vector(vs))
        } else {
            unreachable!()
        }
    }

    fn read_expr(&mut self) -> ReadResult<Datum> {
        if let Some(curr) = self.src.next() {
            match curr {
                Token::Ident(i) | Token::Prim(i) => {
                    if self.case_insensitive {
                        Ok(Datum::Symbol(i.to_ascii_lowercase()))
                    } else {
                        Ok(Datum::Symbol(i.clone()))
                    }
                }
                Token::Bool(b) => Ok(Datum::Bool(*b)),
                Token::Fixnum(f) => Ok(Datum::Fixnum(*f)),
                Token::Char(c) => Ok(Datum::Char(*c)),
                Token::Str(s) => Ok(Datum::Str(s.clone())),
                Token::Ellipses => Ok(Datum::Ellipses),
                Token::Comma | Token::CommaAt | Token::Quote | Token::Quasi => {
                    self.read_quote(curr.clone())
                }
                Token::RParen => Err(ReadError::UnexpectedListTerminator(curr.clone())),
                Token::RBracket => {
                    if self.bracket_paren {
                        Err(ReadError::UnexpectedListTerminator(curr.clone()))
                    } else {
                        Err(ReadError::UnknownSymbol(curr.clone()))
                    }
                }
                Token::BrackVecLParen => {
                    if self.bracket_paren {
                        self.read_vector(Token::LBracket)
                    } else {
                        Err(ReadError::UnknownSymbol(curr.clone()))
                    }
                }
                Token::VecLParen => self.read_vector(Token::LParen),
                Token::ByteVecLParen => self.read_bytevector(),
                Token::LParen => self.read_sexpr(curr.clone()),
                Token::LBracket => {
                    if self.bracket_paren {
                        self.read_sexpr(curr.clone())
                    } else {
                        Err(ReadError::UnknownSymbol(curr.clone()))
                    }
                }
                Token::Eof => Ok(Datum::Eof),
                Token::Hash => {
                    todo!("\nImplement reading from hash\n+ Vectors?\n+ keywords?\n+ syntax?\n")
                }
                Token::Dot => Err(ReadError::UnknownSymbol(curr.clone())),
                _ => Err(ReadError::UnknownSymbol(curr.clone())),
            }
        } else {
            Ok(Datum::Eof)
        }
    }

    /// Bootstrapping `read` function, which implements some basics so as to implement
    /// a `read` function in scheme;
    /// in other words
    /// + `read` >
    /// + comptime Rust representation `Datum` >
    /// + special forms into `Ast` >
    /// + execute scheme `read` procedure
    pub fn read(&mut self) -> Result<Datum, ReadError> {
        if let Some(curr) = self.src.peek() {
            let mut prgrm = Vec::new();

            loop {
                match self.read_expr() {
                    Ok(Datum::Eof) => break,
                    Ok(d) => prgrm.push(d),
                    Err(e) => return Err(e),
                }
            }

            Ok(Datum::List(prgrm))
        } else {
            Ok(Datum::Eof)
        }
    }
}
