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
    Ref(String),
    Integer(i64),
    Double(f64),
    /// Double quoted string: i.e: "string"
    String(String),
    Char(char),
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Ref(n) => n.fmt(f),
            Literal::Integer(num) => num.fmt(f),
            Literal::Double(double) => double.fmt(f),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Char(c) => write!(f, "'{}'", c),
            Literal::Bool(b) => b.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    /// '.'
    Dot,
    /// '::'
    DoubleColon,
    /// Left parenthesis `)`
    LParen,
    /// Right parenthesis `)`
    RParen,
    /// Left bracket `[`
    LBracket,
    /// Right bracket `]`
    RBracket,
    Comma,
    Backslash,
}

impl Display for Punctuation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Punctuation::Dot => ".",
            Punctuation::DoubleColon => "::",
            Punctuation::LParen => "(",
            Punctuation::RParen => ")",
            Punctuation::LBracket => "[",
            Punctuation::RBracket => "]",
            Punctuation::Comma => ",",
            Punctuation::Backslash => "\\",
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
        match self {
            Sym::EOF => write!(f, "'end of file'"),
            Sym::Literal(l) => l.fmt(f),
            Sym::Punctuation(p) => p.fmt(f),
            Sym::Id(i) => i.fmt(f),
            Sym::Keyword(k) => k.fmt(f),
            Sym::Whitespace => write!(f, "<whitespace>"),
            Sym::Eq => write!(f, "'='"),
            Sym::Underscore => write!(f, "'_'"),
        }
    }
}
