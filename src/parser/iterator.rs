use regex::Regex;

use crate::{Markdown, MarkdownType};
use crate::parser::markdown::{MarkdownVisitable, Visitable};
use crate::parser::visitor::{Visitor, VisitorBase};

pub struct MarkdownIterator<V: Visitor + Clone, A: Visitable<V>> {
    visitor: V,
    visitable_list: Vec<A>,
}

impl<V: Visitor + Clone, A: Visitable<V>> MarkdownIterator<V, A> {
    pub fn create(visitor: V, visitable_list: Vec<A>) -> Self {
        Self { visitor, visitable_list }
    }

    pub fn parse_str(&self, input: String) -> Vec<Markdown> {
        let mut result: Vec<Markdown> = vec![];
        let input_vec: Vec<&str> = input
            .split('\n')
            .into_iter()
            .collect();

        for (index, input) in input_vec.iter().enumerate() {
            for visitable in self.visitable_list.iter() {
                match visitable.accept(self.visitor.clone(), index,input) {
                    Ok(markdown) => result.push(markdown),
                    _ => continue,
                }
            }
        }

        result
    }
}

pub fn get_markdown_iterator(visitor: VisitorBase) -> MarkdownIterator<VisitorBase, MarkdownVisitable> {
    let markdown_h1 = MarkdownVisitable {
        style: MarkdownType::H1,
        regex: Regex::new(r"^#\s").unwrap(),
        is_block: true,
    };

    let markdown_h2 = MarkdownVisitable {
        style: MarkdownType::H2,
        regex: Regex::new(r"^#{2}\s").unwrap(),
        is_block: true,
    };

    let markdown_h3 = MarkdownVisitable {
        style: MarkdownType::H2,
        regex: Regex::new(r"^#{3}\s").unwrap(),
        is_block: true,
    };

    let markdown_h4 = MarkdownVisitable {
        style: MarkdownType::H2,
        regex: Regex::new(r"^#{4}\s").unwrap(),
        is_block: true,
    };

    let markdown_h5 = MarkdownVisitable {
        style: MarkdownType::H2,
        regex: Regex::new(r"^#{5}\s").unwrap(),
        is_block: true,
    };

    let markdown_h6 = MarkdownVisitable {
        style: MarkdownType::H2,
        regex: Regex::new(r"^#{6}\s").unwrap(),
        is_block: true,
    };

    MarkdownIterator::create(visitor, vec![markdown_h1, markdown_h2, markdown_h3, markdown_h4, markdown_h5, markdown_h6])
}
