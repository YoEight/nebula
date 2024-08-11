use nebula_core::{
    ast::{Program, Tag},
    register::Register,
    scope::Scope,
    Loc,
};
use r#gen::generate;
use reduction::reduction;
use value::Value;

pub mod gen;
pub mod reduction;
pub mod value;

pub fn eval(mut prog: Program<Loc>) -> eyre::Result<Value> {
    let mut reg = Register::default();
    let scope = Scope::new();
    let root = prog.exprs.pop().unwrap();
    let root = Tag {
        item: Box::new(root),
        tag: Loc { line: 0, col: 0 },
    };

    let val = generate(&mut reg, scope, root)?;
    reduction(&mut reg, val)
}
