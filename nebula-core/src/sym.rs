use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    If,
    Then,
    Else,
    Let,
    Where,
}

impl Keyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::If => "if",
            Keyword::Then => "then",
            Keyword::Else => "else",
            Keyword::Let => "let",
            Keyword::Where => "where",
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Double(f64),
    /// Double quoted string: i.e: "string"
    String(String),
    Char(char),
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Literal::Integer(num) => num.to_string(),
            Literal::Double(double) => double.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Char(c) => format!("'{}'", c),
            Literal::Bool(b) => b.to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    /// '.'
    Dot,
    /// ':'
    Colon,
    /// Left parenthesis `)`
    LParen,
    /// Right parenthesis `)`
    RParen,
    /// Left bracket `[`
    LBracket,
    /// Right bracket `]`
    RBracket,
    /// Pipe `|`
    Comma,
}

impl Display for Punctuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Punctuation::Dot => ".",
            Punctuation::Colon => ":",
            Punctuation::LParen => "(",
            Punctuation::RParen => ")",
            Punctuation::LBracket => "[",
            Punctuation::RBracket => "]",
            Punctuation::Comma => ",",
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sym {
    /// An end-of-file marker, not a real token
    EOF,
    // Type
    Literal(Literal),
    // Identifier
    Id(String),
    Punctuation(Punctuation),
    /// A keyword
    Keyword(Keyword),
    /// Whitespace (space, tab, etc)
    Whitespace,
    /// Equality operator `=`
    Eq,
    Underscore,
}

impl Display for Sym {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Sym::EOF => "EOF",
            Sym::Literal(l) => return write!(f, "{}", l),
            Sym::Punctuation(p) => return write!(f, "{}", p),
            Sym::Id(i) => i.as_str(),
            Sym::Keyword(k) => k.as_str(),
            Sym::Whitespace => " ",
            Sym::Eq => "=",
            Sym::Underscore => "_",
        };

        write!(f, "{}", str)
    }
}
