#[macro_export]
macro_rules! ast {
    ($($x:expr);*;) => {
        vec![$($x),*]
    };
}

// A decl used in global scope
#[macro_export]
macro_rules! gdecl {
    ($aident:ident = $aexpr:expr) => {
        GlobalStatement::Decl(Identifier(stringify!($aident)), expr!($aexpr))
    };
}

// A simple assingment
macro_rules! assing {
    ($aident:ident = $aexpr:expr) => {
        St
    };
}

// Turns literals into arcl expressions
#[macro_export]
macro_rules! expr {
    ($x:expr) => {
        ToExpr::as_expr($x)
    };
}