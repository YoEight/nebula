use crate::sym::{Keyword, Literal, Punctuation, Sym};
use crate::Loc;
use std::iter::Peekable;
use std::ops::Neg;
use std::str::Chars;

enum Number {
    Integer(i64),
    Double(f64),
}

impl Number {
    fn negate(self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(i.neg()),
            Self::Double(d) => Self::Double(d.neg()),
        }
    }

    fn into_sym(self) -> Sym {
        match self {
            Self::Integer(i) => Sym::Literal(Literal::Integer(i)),
            Self::Double(d) => Sym::Literal(Literal::Double(d)),
        }
    }
}

struct Characters<'a> {
    peekable: Peekable<Chars<'a>>,
    pub line: u64,
    pub col: u64,
}

impl<'a> Characters<'a> {
    pub fn next(&mut self) -> Option<char> {
        match self.peekable.next() {
            None => None,
            Some(s) => {
                if s == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }

                Some(s)
            }
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.peekable.peek()
    }

    pub fn loc(&self) -> Loc {
        Loc {
            line: self.line,
            col: self.col,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub item: Sym,
    pub loc: Loc,
}

impl Token {
    pub fn item(&self) -> &Sym {
        &self.item
    }
}

pub struct Tokenizer<'a> {
    query: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Tokenize the statement and produce a vector of tokens with location information
    pub fn tokenize(&self) -> eyre::Result<Vec<Token>> {
        let mut state = Characters {
            peekable: self.query.chars().peekable(),
            line: 1,
            col: 1,
        };

        let mut tokens: Vec<Token> = vec![];
        let mut pos = state.loc();

        while let Some(item) = self.next_token_item(&mut state)? {
            tokens.push(Token { item, loc: pos });

            pos = state.loc();
        }

        tokens.push(Token {
            item: Sym::EOF,
            loc: pos,
        });

        Ok(tokens)
    }

    fn next_token_item(&self, chars: &mut Characters) -> eyre::Result<Option<Sym>> {
        let prev_loc = chars.loc();
        match chars.peek() {
            None => Ok(None),
            Some(ch) => match ch {
                _ if ch.is_ascii_whitespace() => {
                    chars.next();
                    while let Some(c) = chars.peek() {
                        if !c.is_whitespace() {
                            break;
                        }

                        chars.next();
                    }

                    Ok(Some(Sym::Whitespace))
                }

                ':' => {
                    chars.next();
                    let loc = chars.loc();

                    if let Some(':') = chars.next() {
                        return Ok(Some(Sym::Punctuation(Punctuation::DoubleColon)));
                    }

                    eyre::bail!("{} expected ':'", loc);
                }

                '.' => self.consume(chars, Sym::Punctuation(Punctuation::Dot)),
                ',' => self.consume(chars, Sym::Punctuation(Punctuation::Comma)),
                '\\' => self.consume(chars, Sym::Punctuation(Punctuation::Backslash)),
                '[' => self.consume(chars, Sym::Punctuation(Punctuation::LBracket)),
                ']' => self.consume(chars, Sym::Punctuation(Punctuation::RBracket)),
                '(' => self.consume(chars, Sym::Punctuation(Punctuation::LParen)),
                ')' => self.consume(chars, Sym::Punctuation(Punctuation::RParen)),
                '_' => self.consume(chars, Sym::Underscore),

                '<' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        return self.consume(chars, Sym::Id("<=".to_string()));
                    }

                    Ok(Some(Sym::Id("<".to_string())))
                }

                '>' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        return self.consume(chars, Sym::Id(">=".to_string()));
                    }

                    Ok(Some(Sym::Id(">".to_string())))
                }

                '"' => {
                    let mut string = String::new();

                    chars.next();

                    while let Some(ch) = chars.peek() {
                        if *ch == '"' {
                            return self.consume(chars, Sym::Literal(Literal::String(string)));
                        }

                        if *ch == '\n' {
                            eyre::bail!("{} string literal is malformed", chars.loc());
                        }

                        string.push(*ch);
                        chars.next();
                    }

                    eyre::bail!("{} incomplete string literal", chars.loc());
                }

                '+' => self.consume(chars, Sym::Id("+".to_string())),
                '*' => self.consume(chars, Sym::Id("*".to_string())),

                '-' => {
                    if let Some(ch) = chars.peek() {
                        if ch.is_ascii_digit() {
                            let num = self.parse_number(chars)?.negate();
                            return Ok(Some(num.into_sym()));
                        }
                    }

                    self.consume(chars, Sym::Id("-".to_string()))
                }

                '=' => {
                    chars.next();
                    if let Some('=') = chars.peek() {
                        return self.consume(chars, Sym::Id("==".to_string()));
                    }

                    Ok(Some(Sym::Eq))
                }

                _ if ch.is_ascii_alphabetic() => {
                    let mut ident = String::new();

                    ident.push(*ch);
                    chars.next();

                    while let Some(ch) = chars.peek() {
                        if !ch.is_ascii_alphanumeric() && *ch != '_' {
                            break;
                        }

                        ident.push(*ch);
                        chars.next();
                    }

                    match ident.as_str() {
                        "true" => Ok(Some(Sym::Literal(Literal::Bool(true)))),
                        "false" => Ok(Some(Sym::Literal(Literal::Bool(false)))),
                        "if" => Ok(Some(Sym::Keyword(Keyword::If))),
                        "then" => Ok(Some(Sym::Keyword(Keyword::Then))),
                        "else" => Ok(Some(Sym::Keyword(Keyword::Else))),
                        _ => Ok(Some(Sym::Id(ident))),
                    }
                }

                _ if ch.is_ascii_digit() => {
                    let num = self.parse_number(chars)?;
                    Ok(Some(num.into_sym()))
                }

                x => eyre::bail!("{} unexpected symbol '{}'", prev_loc, x),
            },
        }
    }

    fn parse_number(&self, chars: &mut Characters) -> eyre::Result<Number> {
        let mut num = String::new();

        let ch = chars.next().unwrap();
        num.push(ch);

        let mut is_double = false;
        while let Some(ch) = chars.peek() {
            if !ch.is_ascii_digit() && *ch != '.' {
                break;
            }

            if *ch == '.' {
                if !is_double {
                    is_double = true;
                } else {
                    eyre::bail!("{} invalid number format", chars.loc());
                }
            }

            num.push(*ch);
            chars.next();
        }

        if is_double {
            let dou = num
                .parse::<f64>()
                .map_err(|e| eyre::eyre!("{} invalid float number: {}", chars.loc(), e))?;

            Ok(Number::Double(dou))
        } else {
            let num = num
                .parse::<i64>()
                .map_err(|e| eyre::eyre!("{} invalid integer number: {}", chars.loc(), e))?;

            Ok(Number::Integer(num))
        }
    }

    fn consume(&self, chars: &mut Characters, item: Sym) -> eyre::Result<Option<Sym>> {
        chars.next();
        Ok(Some(item))
    }
}
