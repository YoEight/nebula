pub mod ast;
pub mod lexer;
pub mod parser;
pub mod sym;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Loc {
    pub line: u64,
    pub col: u64,
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
