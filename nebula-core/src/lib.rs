use ast::Program;
use lexer::Tokenizer;
use parser::Parser;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod register;
pub mod scope;
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

pub fn parse(code: impl AsRef<str>) -> eyre::Result<Program<Loc>> {
    let tokenizer = Tokenizer::new(code.as_ref());
    let tokens = tokenizer.tokenize()?;
    let parser = Parser::new(tokens.as_ref());

    parser.parse()
}
