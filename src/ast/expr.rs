use core::panic;
use std::fmt::Display;

#[derive(Debug)]
pub enum Expr {
    Int(i32),
    Str(String)
}

pub trait ToExpr {
    fn as_expr(self) -> Expr;
}

impl ToExpr for i32 {
    fn as_expr(self) -> Expr {
        Expr::Int(self)
    }
}

impl ToExpr for &str {
    fn as_expr(self) -> Expr {
        Expr::Str(self.to_string())
    }
}


pub trait FromExpr<T> {
    fn into(self) -> T;
}


impl FromExpr<i32> for Expr {
    fn into(self) -> i32 {
        match self {
            Expr::Int(i) => i,
            e => panic!("Can't cast {:?} to i32", e)
        }
    }
}

impl FromExpr<String> for Expr {
    fn into(self) -> String {
        match self {
            Expr::Str(s) => s,
            e => panic!("Can't cast {:?} to &str", e)
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match &self {
            Expr::Int(i) => i.to_string(),
            Expr::Str(s) => s.to_string(),
        };

        write!(f, "{as_str}")
    }
}