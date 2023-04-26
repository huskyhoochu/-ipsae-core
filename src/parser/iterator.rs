use crate::Markdown;
use crate::parser::markdown::Visitable;
use crate::parser::parse::Visitor;

pub struct StringIterator<V: Visitor + Clone, A: Visitable<V>> {
    visitor: V,
    visitable_list: Vec<A>,
}

impl<V: Visitor + Clone, A: Visitable<V>> StringIterator<V, A> {
    pub fn create(visitor: V, visitable_list: Vec<A>) -> Self {
        Self { visitor, visitable_list }
    }

    pub fn parse_str(&self, input: String) -> Vec<Markdown> {
        let mut result: Vec<Markdown> = vec![];
        let input_vec: Vec<&str> = input
            .split('\n')
            .into_iter()
            .collect();

        for input in input_vec {
            for visitable in self.visitable_list.iter() {
                match visitable.accept(self.visitor.clone(), input) {
                    Ok(markdown) => result.push(markdown),
                    _ => continue,
                }
            }
        }

        result
    }
}

