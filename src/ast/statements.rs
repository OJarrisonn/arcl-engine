use crate::prelude::{Expr, Identifier, AssignType, Var};

#[derive(Debug)]
pub enum GlobalStatement<'a> {
    Decl(Identifier<'a>, Expr),
    Cons(),
    Main(Vec<Expr>)
}

#[derive(Debug)]
pub enum Statement<'a> {
    Decl(Var, Identifier<'a>, AssignType, Expr),
    Assignment(Identifier<'a>, AssignType, Expr)
}