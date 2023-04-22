#[cfg(test)]
mod tests {
    use ipsae::{Markdown, MarkdownType};

    #[test]
    fn it_works() {
        let origin_string = "# hello\nmy friend!".to_string();
        let result = ipsae::render(origin_string);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Markdown { style: MarkdownType::H1, content: "hello".to_string() })
    }
}
