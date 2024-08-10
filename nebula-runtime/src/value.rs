use std::fmt::Display;

use nebula_core::scope::Scope;

#[derive(Clone)]
pub enum Value {
    Uninitialized,
    Var(String),
    Integer(i64),
    Double(f64),
    Bool(bool),
    String(String),
    Char(char),
    Fun {
        scope: Scope,
        name: String,
        body: Box<Value>,
    },
    App {
        lhs: Box<Value>,
        rhs: Box<Value>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Uninitialized => write!(f, "<uninitialized>"),
            Value::Var(v) => v.fmt(f),
            Value::Integer(i) => i.fmt(f),
            Value::Double(d) => d.fmt(f),
            Value::Bool(b) => b.fmt(f),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Char(c) => write!(f, "'{}'", c),

            Value::Fun { name, body, .. } => {
                write!(f, "\\{}. ", name)?;
                body.fmt(f)
            }

            Value::App { lhs, rhs } => {
                write!(f, "(")?;
                lhs.fmt(f)?;
                write!(f, " ")?;
                rhs.fmt(f)?;
                write!(f, ")")
            }
        }
    }
}
