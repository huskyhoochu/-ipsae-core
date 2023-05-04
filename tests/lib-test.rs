#[cfg(test)]
mod tests {
    use ipsae::{Markdown, MarkdownType};

    #[test]
    fn it_works() {
        let origin_string = "# hello\n## my friend!\nmy name is **harry** potter.\n*avada cadavra!*".to_string();
        let result = ipsae::render(origin_string);

        println!("{:?}", result);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], Markdown {
            style: MarkdownType::H1,
            content: "hello".to_string(),
            children: vec![],
        });
        assert_eq!(result[1], Markdown {
            style: MarkdownType::H2,
            content: "my friend!".to_string(),
            children: vec![],
        });
        assert_eq!(result[2], Markdown {
            style: MarkdownType::P,
            content: "my name is **harry** potter.".to_string(),
            children: vec![
                Markdown {
                    style: MarkdownType::Strong,
                    content: "harry".to_string(),
                    children: vec![],
                }
            ],
        });
        assert_eq!(result[3], Markdown {
            style: MarkdownType::P,
            content: "*avada cadavra!*".to_string(),
            children: vec![
                Markdown {
                    style: MarkdownType::Italic,
                    content: "avada cadavra!".to_string(),
                    children: vec![],
                }
            ],
        });
    }
}
