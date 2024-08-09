use crate::sym::Literal;

#[derive(Debug, PartialEq)]
pub struct Program<A> {
    pub procs: Vec<Tag<Expr<A>, A>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag<I, A> {
    pub item: I,
    pub tag: A,
}

impl<I, A> std::fmt::Display for Tag<I, A>
where
    I: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)
    }
}

impl<I, A> Tag<I, A> {
    pub fn map_item<F, J>(self, fun: F) -> Tag<J, A>
    where
        F: FnOnce(I) -> J,
    {
        Tag {
            item: fun(self.item),
            tag: self.tag,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<A> {
    Ident(String),
    Literal(Literal),
    Fun(String, Tag<Box<Tag<Expr<A>, A>>, A>),
    App(Tag<Box<Tag<Expr<A>, A>>, A>, Box<Tag<Expr<A>, A>>),
}
