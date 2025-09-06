#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    I64,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64, Type),
    Var(String, Type),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
