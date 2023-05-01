use regex::Regex;

use crate::parser::visitor::Visitor;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MarkdownType {
    H1,
    H2,
    H3,
    P,
    Strong,
    Italic,
}

#[derive(Debug, PartialEq)]
pub struct Markdown {
    pub index: usize,
    pub style: MarkdownType,
    pub content: String,
}

pub trait Visitable<V: Visitor> {
    fn accept(&self, visitor: V, index: usize, content: &str) -> Result<Markdown,  &'static str>;
}

pub struct MarkdownVisitable {
    pub style: MarkdownType,
    pub regex: Regex,
}

impl<V: Visitor> Visitable<V> for MarkdownVisitable {
    fn accept(&self, visitor: V, index: usize, content: &str) -> Result<Markdown,  &'static str> {
        visitor.visit(index, self.regex.clone(), self.style.clone(), content)
    }
}
