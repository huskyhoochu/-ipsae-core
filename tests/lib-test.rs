#[cfg(test)]
mod tests {
    use ipsae::{Markdown, MarkdownType};

    #[test]
    fn it_works() {
        let origin_string = "# hello\n## my friend!\nmy name is harry.".to_string();
        let result = ipsae::render(origin_string);

        println!("{:?}", result);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Markdown {
            style: MarkdownType::H1,
            content: Option::from("hello".to_string()),
            children: vec![],
        });
        assert_eq!(result[1], Markdown {
            style: MarkdownType::H2,
            content: Option::from("my friend!".to_string()),
            children: vec![],
        });
        assert_eq!(result[2], Markdown {
            style: MarkdownType::P,
            content: Option::from("my name is harry.".to_string()),
            children: vec![],
        });
    }
}
