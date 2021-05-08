
use super::tokentype;

pub enum State {
    IdentifierState,
    WhitespaceState,
    ErrorState,
}

pub fn next_state(curr: char) -> State {
    if curr.is_alphabetic() || curr == '_' {
        State::IdentifierState
    } else if curr.is_whitespace() {
        State::WhitespaceState
    } else {
        State::ErrorState
    }
}

pub fn identifier(curr: char) -> (tokentype::TokenType, State) {
    if curr.is_alphanumeric() || curr == '_' {
        (tokentype::TokenType::None, State::IdentifierState)
    } else {
        (tokentype::TokenType::Identifier, next_state(curr))
    }
}

pub fn whitespace(curr: char) -> (tokentype::TokenType, State) {
    if curr.is_whitespace() {
        (tokentype::TokenType::None, State::WhitespaceState)
    } else {
        (tokentype::TokenType::Whitespace, next_state(curr))
    }
}

pub fn error_state(curr: char) -> (tokentype::TokenType, State) {
    if curr == '\0' {
        (tokentype::TokenType::Error, State::ErrorState)
    } else {
        (tokentype::TokenType::None, State::ErrorState)
    }
}

pub fn step_state(curr: char, state: State) -> (tokentype::TokenType, State) {
    match state {
        State::IdentifierState => identifier(curr),
        State::WhitespaceState => whitespace(curr),
        State::ErrorState => error_state(curr),
    }
}