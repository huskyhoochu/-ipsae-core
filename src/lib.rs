
use regex::Regex;

use parser::iterator::StringIterator;
pub use parser::markdown::{Markdown, MarkdownType};
use parser::parse::VisitorBase;

use crate::parser::markdown::MarkdownVisitable;

mod parser;


pub fn render(origin_string: String) -> Vec<Markdown> {
    let visitor = VisitorBase;

    let markdown_h1 = MarkdownVisitable {
        style: MarkdownType::H1,
        regex: Regex::new(r"^#\s").unwrap(),
    };

    let markdown_h2 = MarkdownVisitable {
        style: MarkdownType::H2,
        regex: Regex::new(r"^#{2}\s").unwrap(),
    };

    let iterator = StringIterator::create(visitor, vec![markdown_h1, markdown_h2]);

    iterator.parse_str(origin_string)
}
