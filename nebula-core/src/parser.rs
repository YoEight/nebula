use crate::ast::{Expr, Program, Tag};
use crate::lexer::Token;
use crate::sym::{Keyword, Literal, Punctuation, Sym};
use crate::Loc;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Clone)]
pub struct ParserState<'a> {
    peekable: Peekable<Iter<'a, Token>>,
}

impl<'a> ParserState<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            peekable: tokens.iter().peekable(),
        }
    }

    pub fn look_ahead(&mut self) -> &'a Token {
        self.peekable.peek().unwrap()
    }

    pub fn shift(&mut self) -> &'a Token {
        self.peekable.next().unwrap()
    }

    pub fn skip_spaces(&mut self) {
        loop {
            match self.look_ahead().item() {
                Sym::Whitespace => {
                    self.shift();
                }

                _ => break,
            }
        }
    }

    pub fn next_keyword(&mut self, key: Keyword) -> bool {
        self.next_sym(Sym::Keyword(key))
    }

    fn expect(&mut self, expected: Sym) -> eyre::Result<()> {
        let loc = self.loc();
        let token = self.shift();

        if &expected != token.item() {
            eyre::bail!(
                "{} expected {} but got {} instead",
                loc,
                expected,
                token.item(),
            );
        }

        Ok(())
    }

    fn expect_ident(&mut self) -> eyre::Result<String> {
        let loc = self.loc();
        let token = self.shift();

        if let Sym::Id(n) = token.item() {
            return Ok(n.clone());
        }

        eyre::bail!(
            "{} expected an identifier but got {} instead",
            loc,
            token.item(),
        );
    }

    // fn expect_keyword(&mut self, expected: Keyword) -> eyre::Result<()> {
    //     self.expect(Sym::Keyword(expected))
    // }

    fn expect_punctuation(&mut self, expected: Punctuation) -> eyre::Result<()> {
        self.expect(Sym::Punctuation(expected))
    }

    pub fn followed_by<F>(&mut self, fun: F) -> bool
    where
        F: FnOnce(&Sym) -> bool,
    {
        fun(self.look_ahead().item())
    }

    pub fn followed_by_keyword(&mut self, keyword: Keyword) -> bool {
        self.followed_by(|sym| sym == &Sym::Keyword(keyword))
    }

    pub fn next_sym(&mut self, sym: Sym) -> bool {
        self.followed_by(|s| s == &sym)
    }

    pub fn next_punct(&mut self, expected: Punctuation) -> bool {
        self.next_sym(Sym::Punctuation(expected))
    }

    pub fn parse_expr(&mut self) -> eyre::Result<Expr<Loc>> {
        let token = self.look_ahead();

        match token.item() {
            Sym::Id(name) => {
                self.shift();
                Ok(Expr::Literal(Literal::Ref(name.clone())))
            }

            Sym::Literal(l) => {
                self.shift();
                Ok(Expr::Literal(l.clone()))
            }

            // Function
            Sym::Punctuation(Punctuation::Backslash) => {
                let start = self.loc();
                self.shift();
                self.skip_spaces();
                let name = self.expect_ident()?;
                self.skip_spaces();
                self.expect_punctuation(Punctuation::Dot)?;
                self.skip_spaces();
                let body_loc = self.loc();
                let body = self.parse_expr()?;

                Ok(Expr::Fun(
                    name,
                    Tag {
                        item: Box::new(Tag {
                            item: body,
                            tag: body_loc,
                        }),
                        tag: start,
                    },
                ))
            }

            // Application
            Sym::Punctuation(Punctuation::LParen) => {
                let start = self.loc();
                self.shift();
                self.skip_spaces();
                let fun_loc = self.loc();
                let fun = self.parse_expr()?;
                self.expect(Sym::Whitespace)?;
                let arg_loc = self.loc();
                let arg = self.parse_expr()?;
                self.skip_spaces();
                self.expect_punctuation(Punctuation::RParen)?;
                let end = self.loc();

                Ok(Expr::App(
                    Tag {
                        item: Box::new(Tag {
                            item: fun,
                            tag: fun_loc,
                        }),

                        tag: start,
                    },
                    Tag {
                        item: Box::new(Tag {
                            item: arg,
                            tag: arg_loc,
                        }),
                        tag: end,
                    },
                ))
            }

            _ => eyre::bail!("{} unexpected token {}", self.loc(), token.item()),
        }
    }

    pub fn loc(&mut self) -> Loc {
        self.look_ahead().loc
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> eyre::Result<Program<Loc>> {
        let mut state = ParserState::new(self.tokens);

        let mut exprs = Vec::new();
        let loc = state.loc();

        exprs.push(Tag {
            item: state.parse_expr()?,
            tag: loc,
        });

        state.skip_spaces();
        state.expect(Sym::EOF)?;

        Ok(Program { exprs })
    }
}
