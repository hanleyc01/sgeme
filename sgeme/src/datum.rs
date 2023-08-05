//! Module specifying the results of a the `read` function, following R7RS standards

/// The result of the `read::Read` function
#[derive(Debug, Clone)]
pub enum Datum {
    Quote(AbbrevPrefix, Box<Self>),
    Bool(bool),
    ByteVector(Vec<u8>),
    Char(char),
    DottedList(Vec<Self>, Box<Self>),
    Fixnum(i32),
    Label(u32),
    List(Vec<Self>),
    Set(u32, Box<Self>),
    Str(String),
    Symbol(String),
    Vector(Vec<Self>),
    Ellipses,
    Null,
    Undefined,
    Eof,
}

impl Datum {
    pub fn is_list(&self) -> bool {
        match self {
            Self::List(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _ => false,
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Self::Str(s) => s.clone(),
            _ => unreachable!(),
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Self::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn get_symbol_name(&self) -> String {
        match self {
            Self::Symbol(s) => s.to_owned(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AbbrevPrefix {
    Quote,
    Quasi,
    Comma,
    CommaAt,
}
