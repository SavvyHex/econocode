#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    I64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64, Type),
    Var(String, Type),
    Assign(String, Box<Expr>, Type),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Cmp(CmpOp, Box<Expr>, Box<Expr>),
    Read(String, Type),
    Block(Vec<Expr>),
    IfElse {
        cond: Box<Expr>,
        then_branch: Vec<Expr>,
        else_branch: Option<Vec<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Vec<Expr>,
    },
}
