use std::collections::HashMap;

use self::{core::Var, expr::Expr, statements::GlobalStatement};

pub mod statements;
pub mod expr;
pub mod core;

/// A map to holds the value assigned to a symbol
type DeclTable = HashMap<String, (Var, Expr)>;

/// Receives an AST and evaluates it node by node, where each node is a GlobalStatement
pub fn evaluate(ast: Vec<GlobalStatement>) {
    let mut decl_table: DeclTable = HashMap::new();

    for node in ast {
        match node {
            GlobalStatement::Decl(id, expr) => {
                    decl_symbol(&mut decl_table, id.0.to_owned(), Var::Const, expr);
            },
            GlobalStatement::Cons() => todo!(),
            GlobalStatement::Main(_) => todo!(),
        };
    }

    println!("{:#?}", decl_table)
}

/// Declares a symbol assing it to an expression
pub fn decl_symbol(dt: &mut DeclTable, id: String, var: Var, expr: Expr) {
    dt.insert(id, (var, expr));
}

/// Changes the value of a symbol from an expression
pub fn set_symbol(dt: &mut DeclTable, id: String, expr: Expr) -> Result<(), String> {
    let decl = dt.get_mut(&id);
    match decl {
        Some((v, e)) => match v {
            Var::Var => {
                *e = expr;
                Ok(())
            },
            Var::Const => {
                Err(format!("Trying to change value of constant {id}"))
            },
        },
        None => Err(format!("Trying to assign to undeclared symbol {id}. Try: decl var {id} = {expr}")),
    }
}

/// Obtains the value of a symbol as an expression
pub fn get_symbol(dt: &mut DeclTable, id: String) -> Option<&Expr> {
    match dt.get(&id) {
        None => None,
        Some((_, e)) => Some(e)
    }
}
