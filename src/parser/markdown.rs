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
    pub style: MarkdownType,
    pub content: String,
    pub children: Vec<Markdown>,
}

pub trait Visitable<V: Visitor> {
    fn accept(&self, visitor: V, content: &str) -> Option<Markdown>;
}

pub struct MarkdownVisitable {
    pub style: MarkdownType,
    pub regex: Regex,
    pub is_block: bool,
}

impl<V: Visitor> Visitable<V> for MarkdownVisitable {
    fn accept(&self, visitor: V, content: &str) -> Option<Markdown> {
        if self.regex.is_match(content) {
            let split_content: Vec<&str> = self.regex.split(content).collect();
            let capture_content = self.regex.captures(content).unwrap().get(3).map_or("", |m| m.as_str());
            if self.style == MarkdownType::P {
                return Some(visitor.visit(self.style.clone(), content));
            }
            if self.is_block == true {
                return Some(visitor.visit(self.style.clone(), split_content[1]));
            }
            return Some(visitor.visit(self.style.clone(), capture_content));
        }

        None
    }
}
