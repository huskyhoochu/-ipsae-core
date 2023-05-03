use crate::{Markdown, MarkdownType};

pub trait Visitor {
    fn visit(&self, style: MarkdownType, content: &str) -> Markdown;
}

#[derive(Copy, Clone)]
pub struct VisitorBase;

impl Visitor for VisitorBase {
    fn visit(&self, style: MarkdownType, content: &str) -> Markdown {
        return Markdown {
            style: style.into(),
            content: Some(content.to_string()),
            children: vec![],
        }
    }
}