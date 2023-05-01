pub use parser::markdown::{Markdown, MarkdownType};
use parser::visitor::VisitorBase;

use crate::parser::iterator::get_markdown_iterator;

mod parser;


pub fn render(origin_string: String) -> Vec<Markdown> {
    let visitor = VisitorBase;
    let iterator = get_markdown_iterator(visitor);

    iterator.parse_str(origin_string)
}
