use nebula_core::{
    ast::{Expr, Tag},
    register::Register,
    scope::Scope,
    sym::Literal,
    Loc,
};

use crate::value::Value;

pub fn generate(
    reg: &mut Register<Value>,
    scope: Scope,
    expr: Tag<Box<Tag<Expr<Loc>, Loc>>, Loc>,
) -> eyre::Result<Value> {
    match (*expr.item).item {
        Expr::Literal(l) => Ok(generate_literal(l)),
        Expr::Fun(n, b) => generate_fun(reg, scope, n, b),
        Expr::App(l, r) => generate_app(reg, scope, l, r),
    }
}

fn generate_literal(lit: Literal) -> Value {
    match lit {
        Literal::Ref(v) => Value::Var(v),
        Literal::Integer(i) => Value::Integer(i),
        Literal::Double(d) => Value::Double(d),
        Literal::String(s) => Value::String(s),
        Literal::Char(c) => Value::Char(c),
        Literal::Bool(b) => Value::Bool(b),
    }
}

fn generate_fun(
    reg: &mut Register<Value>,
    scope: Scope,
    name: String,
    body: Tag<Box<Tag<Expr<Loc>, Loc>>, Loc>,
) -> eyre::Result<Value> {
    let scope = scope.inherits();

    if !reg.register(&scope, &name, Value::Uninitialized) {
        eyre::bail!(
            "{} variable '{}' is already introduced in that scope",
            body.tag,
            name
        );
    }

    let body = generate(reg, scope.clone(), body)?;

    Ok(Value::Fun {
        name: name.to_string(),
        body: Box::new(body),
        scope,
    })
}

fn generate_app(
    reg: &mut Register<Value>,
    scope: Scope,
    lhs: Tag<Box<Tag<Expr<Loc>, Loc>>, Loc>,
    rhs: Tag<Box<Tag<Expr<Loc>, Loc>>, Loc>,
) -> eyre::Result<Value> {
    let lhs = generate(reg, scope.clone(), lhs)?;
    let rhs = generate(reg, scope, rhs)?;

    Ok(Value::App {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}
