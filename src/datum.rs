#[derive(Debug)]
pub enum Datum {
    Simple(SimpleDatum),
    List(Vec<Self>),
    DottedList(Vec<Self>, Box<Self>),
    Vector(Vec<Self>),
    Abbrev(AbbrevPrefix, Box<Self>),
    Set(u32, Box<Self>),
    Label(u32),
}

#[derive(Debug)]
pub enum SimpleDatum {
    Bool(bool),
    Fixnum(i32),
    Char(char),
    Str(String),
    Symbol(String),
    ByteVector(Vec<u8>),
}

#[derive(Debug)]
pub enum AbbrevPrefix {
    Quote,
    Quasi,
    Comma,
    CommaAt,
}
