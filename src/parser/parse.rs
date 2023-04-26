use regex::Regex;

use crate::{Markdown, MarkdownType};

pub trait Visitor {
    fn visit(&self, regex: Regex, style: MarkdownType, content: &str) -> Result<Markdown, &'static str>;
}

#[derive(Copy, Clone)]
pub struct VisitorBase;

impl Visitor for VisitorBase {
    fn visit(&self, regex: Regex, style: MarkdownType, content: &str) -> Result<Markdown, &'static str> {
        if regex.is_match(content) {
            let split_line: Vec<&str> = regex.split(content).collect();
            return Ok(Markdown {
                style: style.into(),
                content: split_line[1].to_string(),
            });
        }

        Err("")
    }
}