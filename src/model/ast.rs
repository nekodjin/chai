#[derive(Debug, Clone)]
pub struct Module {
    pub members: Vec<Member>,
}

#[derive(Debug, Clone)]
pub enum Member {
    Func(Func),
}

#[derive(Debug, Clone)]
pub struct Func {
    pub visibility: Visibility,
    pub name: String,
    pub body: Vec<Stmt>,
    pub tail: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDecl(Decl),
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub mutable: bool,
    pub name: String,
    pub type_annot: Option<String>,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    FuncCall { func: Box<Expr>, args: Vec<Expr> },
    Assign { target: String, value: Box<Expr> },
}
