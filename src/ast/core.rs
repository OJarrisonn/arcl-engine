use super::expr::Expr;

#[derive(Debug)]
pub struct Identifier<'a>(pub &'a str);

#[derive(Debug)]
pub enum Assignment<'a> {
    Copy(Identifier<'a>, Expr)
}

#[derive(Debug)]
pub enum Var{
    Var,
    Const
}

#[derive(Debug)]
pub enum Access{
    Priv,
    Read,
    Write
}

#[derive(Debug)]
pub enum AssignType {
    Copy, 
    Point,
    DeepCopy
}