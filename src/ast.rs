#[derive(Debug)]
pub struct ParsedModule {
    name: String,
    exports: Vec<Export>,
    imports: Vec<Import>,
    stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Def(Def),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Def {
    VarDef(String, Expr),
    FuncDef(String, Vec<String>, Expr),
}

#[derive(Debug)]
pub enum Literal {
    Quote(String),
    Bool(bool),
    Fixnum(i32),
    Vector(Vec<Self>),
    Char(char),
    Str(String),
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    Proc(Box<Self>, Vec<Self>),
    Lambda(Vec<String>, Box<Self>),
    Conditional(Box<Self>, Box<Self>, Box<Self>),
    Assignment(String, Box<Self>),
}

#[derive(Debug)]
pub enum Import {
    LibName(String),
    Only(Box<Self>, Vec<String>),
    Except(Box<Self>, Vec<String>),
    Prefix(Box<Self>, String),
    Rename(Box<Self>, Vec<(String, String)>),
}

#[derive(Debug)]
pub enum Export {
    Name(String),
    Rename(String, String),
}
