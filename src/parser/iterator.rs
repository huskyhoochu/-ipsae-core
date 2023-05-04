use regex::Regex;

use crate::{Markdown, MarkdownType};
use crate::parser::markdown::{MarkdownVisitable, Visitable};
use crate::parser::visitor::{Visitor, VisitorBase};

#[derive(Debug)]
pub struct MarkdownIterator<V: Visitor + Clone, A: Visitable<V>> {
    visitor: V,
    visitable_block_list: Vec<A>,
    visitable_inline_list: Vec<A>,
}

impl<V: Visitor + Clone, A: Visitable<V>> MarkdownIterator<V, A> {
    pub fn create(visitor: V, visitable_block_list: Vec<A>, visitable_inline_list: Vec<A>) -> Self {
        Self { visitor, visitable_block_list, visitable_inline_list }
    }

    pub fn parse_markdown(&self, input: String) -> Vec<Markdown> {
        let mut result: Vec<Markdown> = vec![];
        let input_vec: Vec<&str> = input
            .split('\n')
            .into_iter()
            .collect();

        for input in input_vec {
            for visitable in self.visitable_block_list.iter() {
                match visitable.accept(self.visitor.clone(), input) {
                    Some(markdown) => result.push(markdown),
                    None => continue,
                }
            }
        }

        result
            .into_iter()
            .map(|mut markdown_item| {
                if markdown_item.style == MarkdownType::P {
                    for visitable in self.visitable_inline_list.iter() {
                        match visitable.accept(self.visitor.clone(), markdown_item.content.clone().as_str()) {
                            Some(markdown_child) => {
                                markdown_item.children.push(markdown_child);
                                break;
                            },
                            None => continue,
                        }
                    }
                }
                return markdown_item;
            })
            .collect()
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
        style: MarkdownType::H3,
        regex: Regex::new(r"^#{3}\s").unwrap(),
        is_block: true,
    };

    let markdown_p = MarkdownVisitable {
        style: MarkdownType::P,
        regex: Regex::new(r"^[^#\-]+$").unwrap(),
        is_block: true,
    };

    let markdown_strong = MarkdownVisitable {
        style: MarkdownType::Strong,
        regex: Regex::new(r"((\*){2})([^*].+)((\*){2})").unwrap(),
        is_block: false,
    };

    let markdown_italic = MarkdownVisitable {
        style: MarkdownType::Italic,
        regex: Regex::new(r"((\*){1})([^*].+)((\*){1})").unwrap(),
        is_block: false,
    };

    MarkdownIterator::create(visitor,
                             vec![markdown_h1, markdown_h2, markdown_h3, markdown_p],
                             vec![markdown_strong, markdown_italic])
}
