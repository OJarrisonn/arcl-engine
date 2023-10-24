use arcl_engine::prelude::*;

#[test]
fn global_const() {
    let ast = ast!(
        gdecl!(age = 5);
        gdecl!(id = "i'm jorge harrisonn");
    );

    println!("{:#?}", &ast);

    ast::evaluate(ast);
}