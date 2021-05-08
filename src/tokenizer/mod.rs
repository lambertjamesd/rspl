

mod tokentype;
mod state;

pub trait Source {
    fn get_id(&self) -> u32;
    fn read(&self, at: usize) -> char;
    fn substring(&self, start: usize, end: usize) -> String;
}

pub struct SourceLocation {
    src_id: u32,
    at: u32,
    end: u32,
}

pub struct Token {
    value: String,
    token_type: tokentype::TokenType,
    location: SourceLocation,
}

pub fn tokenize(source: &dyn Source) -> Vec<Token> {
    let src_id = source.get_id();

    let mut tokens = Vec::new();

    let mut curr = source.read(0);
    let mut curr_state = state::next_state(source.read(0));

    let mut has_more = curr != '\0';
    let mut index: usize = 1;
    let mut last_token_start: usize = 0;

    while has_more {
        curr = source.read(index);
        let next = state::step_state(curr, curr_state);

        if !matches!(next.0, tokentype::TokenType::None) {
            if !matches!(next.0, tokentype::TokenType::Whitespace) {
                tokens.push(Token {
                    value: source.substring(last_token_start, index),
                    token_type: next.0,
                    location: SourceLocation {src_id: src_id, at: last_token_start as u32, end: index as u32},
                });
            }

            last_token_start = index;
        }

        curr_state = next.1;

        if curr == '\0' {
            has_more = false;
        }

        index = index + 1;
    }
    
    tokens.push(Token {
        value: "".to_string(),
        token_type: tokentype::TokenType::EOF,
        location: SourceLocation {src_id: src_id, at: index as u32, end: index as u32},
    });

    tokens
}