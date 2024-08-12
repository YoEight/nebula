use nebula_core::register::Register;

use crate::value::Value;

pub fn reduction(reg: &mut Register<Value>, expr: Value) -> eyre::Result<Value> {
    let expr = eta_reduction(reg, expr);
    beta_reduction(reg, expr)
}

fn beta_reduction(reg: &mut Register<Value>, expr: Value) -> eyre::Result<Value> {
    match expr {
        Value::App { lhs, rhs } => match *lhs {
            Value::Fun { scope, name, body } => {
                let body = beta_reduction(reg, *body)?;
                let rhs = beta_reduction(reg, *rhs)?;

                reg.register(&scope, &name, rhs);

                Ok(Value::Fun {
                    scope,
                    name,
                    body: Box::new(body),
                })
            }

            Value::Var(s, name) => {
                let rhs = beta_reduction(reg, *rhs)?;

                Ok(Value::App {
                    lhs: Box::new(Value::Var(s, name)),
                    rhs: Box::new(rhs),
                })
            }

            _ => eyre::bail!("exception: expected a function or a variable"),
        },

        x => Ok(x),
    }
}

fn eta_reduction(reg: &mut Register<Value>, expr: Value) -> Value {
    match expr {
        Value::Fun { scope, name, body } => {
            let body = eta_reduction(reg, *body);

            if let Value::App { lhs, rhs } = body {
                match *rhs {
                    Value::Var(scope, var) => {
                        if name == var {
                            reg.remove(&scope, &name);
                            return *lhs;
                        } else {
                            Value::App {
                                lhs,
                                rhs: Box::new(Value::Var(scope, var)),
                            }
                        }
                    }

                    x => Value::App {
                        lhs,
                        rhs: Box::new(x),
                    },
                }
            } else {
                Value::Fun {
                    scope,
                    name,
                    body: Box::new(body),
                }
            }
        }

        x => x,
    }
}
