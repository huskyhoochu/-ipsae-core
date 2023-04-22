#[cfg(test)]
mod tests {
    use ipsae::{Markdown, MarkdownType};

    #[test]
    fn it_works() {
        let origin_string = "# hello\n## my friend!".to_string();
        let result = ipsae::render(origin_string);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Markdown {
            style: MarkdownType::H1,
            content: "hello".to_string(),
        });
        assert_eq!(result[1], Markdown {
            style: MarkdownType::H2,
            content: "my friend!".to_string(),
        });
    }
}
