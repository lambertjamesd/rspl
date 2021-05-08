
mod tokenizer;

struct StringSource {
    id: u32,
    source: Vec<char>,
}

fn new_string_source(from: String) -> StringSource {
    StringSource {
        id: 0,
        source: from.chars().collect(),
    }
}

impl tokenizer::Source for StringSource {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn read(&self, at: usize) -> char {
        let mut result = 0 as char;

        if at < self.source.len() {
            result = self.source[at]
        }

        return result
    }

    fn substring(&self, start: usize, end: usize) -> String {
        let mut result = String::with_capacity(end - start);

        for index in start..end {
            result.push(self.source[index]);
        }

        result
    }
}

fn main() {
    let source = new_string_source("Hello World".to_string());

    let tokens = tokenizer::tokenize(&source);

    println!("Hello, world!");
}