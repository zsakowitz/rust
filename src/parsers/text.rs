use crate::{
    parser::Parser,
    parser_state::{ParserState, UnknownParserState},
};

pub struct TextParser {
    text: String,
}

impl TextParser {
    pub fn new(text: String) -> TextParser {
        TextParser { text }
    }
}

impl Parser for TextParser {
    type Output = String;

    fn parse(&self, state: &UnknownParserState) -> ParserState<Self::Output> {
        let slice = &state.source[state.index..];

        if slice.starts_with(&self.text) {
            state
                .with_index(state.index + self.text.len())
                .as_ok(self.text.clone())
        } else {
            state.as_err(format!(
                "Expected '{}'; found '{}'.",
                self.text,
                slice[0..10].to_owned()
            ))
        }
    }
}
