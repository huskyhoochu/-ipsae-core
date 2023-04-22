use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq)]
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
}

impl FromStr for Markdown {
    type Err = std::str::Utf8Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let h1_regex = Regex::new(r"^#\s").unwrap();
        match h1_regex.is_match(line) {
            true => {
                let split_line: Vec<&str> = h1_regex.split(line).collect();
                return Ok(Markdown {
                    style: MarkdownType::H1,
                    content: split_line[1].to_string(),
                });
            }
            false => {}
        }

        let h2_regex = Regex::new(r"^#{2}\s").unwrap();
        match h2_regex.is_match(line) {
            true => {
                let split_line: Vec<&str> = h2_regex.split(line).collect();
                return Ok(Markdown {
                    style: MarkdownType::H2,
                    content: split_line[1].to_string(),
                });
            }
            false => {}
        }

        Ok(Markdown {
            style: MarkdownType::P,
            content: line.to_string(),
        })
    }
}

pub fn render(origin_string: String) -> Vec<Markdown> {
    origin_string
        .split('\n')
        .into_iter()
        .map(|line| match Markdown::from_str(line) {
            Ok(markdown) => markdown,
            Err(_) => {
                println!("not valid markdown");
                return Markdown { style: MarkdownType::P, content: line.to_string() };
            }
        }).collect()
}