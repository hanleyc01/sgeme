//! Acceptable tokens according to the R7RS standard (not fully compliant)
pub use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip "[ \t\n\r]+")]
pub enum Token {
    #[regex(r"-?[0-9]+", 
            |lex| lex.slice().parse::<i32>().unwrap())]
    Fixnum(i32),

    #[regex(r#""[^"](\\" | .*)""#,
            |lex| lex.slice().to_owned())]
    Str(String),

    #[regex(r"([a-zA-Z]|!|\$|%|&|\*|/|:|<|=|>|\?|~|_|\^)([a-zA-Z]|!|\$|%|&|\*|/|:|<|=|>|\?|~|_|\^|[0-9]|\.|\+|\-)*",
            |lex| lex.slice().to_owned())]
    Ident(String),

    #[regex(r"(\+|\*|-|/)",
            |lex| lex.slice().to_owned())]
    Prim(String),

    #[token("#\\newline",
            |_| '\n')]
    #[token("#\\space",
            |_| ' ')]
    #[regex(r#"#\."#,
            |lex| lex.slice().parse::<char>().unwrap())]
    Char(char),

    #[token("#t",
            |_| true)]
    #[token("#f",
            |_| false)]
    Bool(bool),

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("#(")]
    VecLParen,

    #[token("#[")]
    BrackVecLParen,

    #[token("#u8(")]
    ByteVecLParen,

    #[token("'")]
    #[token("quote")]
    Quote,

    #[token("`")]
    Quasi,

    #[token(",")]
    Comma,

    #[token(",@")]
    CommaAt,

    #[token("#")]
    Hash,

    #[token(".")]
    Dot,

    #[token("...")]
    Ellipses,

    #[token(";", |lex| {
        let len = lex.remainder().find('\n').unwrap();
        lex.bump(len+1);
        Some(lex.slice().to_owned())
     })]
    Comment(String),

    Eof,
}

impl Token {
    pub fn really_lex(src: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lexer = Token::lexer(&src);

        for token in lexer {
            match token {
                Ok(Self::Comment(_)) => (),
                Err(_) => (),
                Ok(t) => tokens.push(t),
            }
        }

        tokens.push(Self::Eof);
        tokens
    }
}
