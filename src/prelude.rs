pub use crate::ast::{
    statements::{
        Statement,
        GlobalStatement
    }, 
    core::{
        AssignType, 
        Identifier,
        Var,
        Access
    },
    expr::{
        Expr, 
        ToExpr
    }
};

pub use crate::{ast, gdecl, expr};