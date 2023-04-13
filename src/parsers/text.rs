use crate::{Parser, ParserState, UnknownParserState};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextParser<'a> {
    text: &'a str,
}

impl<'a> TextParser<'a> {
    pub fn new(text: &'a str) -> TextParser<'a> {
        TextParser { text }
    }
}

impl<'b> Parser for TextParser<'b> {
    type Output = &'b str;

    fn parse<'a>(&self, state: &'a UnknownParserState<'a>) -> ParserState<'a, Self::Output> {
        let slice = &state.source[state.index..];

        if slice.starts_with(&self.text) {
            ParserState {
                data: Ok(self.text),
                index: state.index + self.text.len(),
                source: state.source,
            }
        } else {
            state.as_err(format!(
                "Expected '{}'; found '{}'.",
                self.text,
                slice[0..(10).min(slice.len())].to_owned()
            ))
        }
    }
}

#[test]
fn successful_parsing_from_start() {
    let binding = ParserState::new("Hello world");
    let state = binding.as_unknown();
    let parser = TextParser::new("Hello");
    let result = parser.parse(&state);

    assert_eq!(
        result,
        ParserState {
            data: Ok("Hello"),
            index: 5,
            source: "Hello world"
        }
    );
}

#[test]
fn failed_parsing_from_start() {
    let binding = ParserState::new("Hello world");
    let state = binding.as_unknown();
    let parser = TextParser::new("Hi");
    let result = parser.parse(&state);

    assert_eq!(
        result,
        ParserState {
            data: Err("Expected 'Hi'; found 'Hello worl'.".to_owned()),
            index: 0,
            source: "Hello world"
        }
    );
}

#[test]
fn successful_parsing_from_middle() {
    let binding = ParserState::new("Hello world");
    let state = binding.as_unknown();
    let parser = TextParser::new("Hello");
    let result = parser.parse(&state);

    assert_eq!(
        result,
        ParserState {
            data: Ok("Hello"),
            index: 5,
            source: "Hello world"
        }
    );

    let state = result.as_unknown();
    let parser = TextParser::new(" ");
    let result = parser.parse(&state);

    assert_eq!(
        result,
        ParserState {
            data: Ok(" "),
            index: 6,
            source: "Hello world"
        }
    );
}

#[test]
fn failed_parsing_from_middle() {
    let binding = ParserState::new("Hello world");
    let state = binding.as_unknown();
    let parser = TextParser::new("Hello");
    let result = parser.parse(&state);

    assert_eq!(
        result,
        ParserState {
            data: Ok("Hello"),
            index: 5,
            source: "Hello world"
        }
    );

    let state = result.as_unknown();
    let parser = TextParser::new("world");
    let result = parser.parse(&state);

    assert_eq!(
        result,
        ParserState {
            data: Err("Expected 'world'; found ' world'.".to_owned()),
            index: 5,
            source: "Hello world"
        }
    );
}
