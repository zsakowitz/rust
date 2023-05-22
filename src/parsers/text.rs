use crate::{Input, InputAndData, Parser};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TextParser<'a> {
    text: &'a str,
}

impl<'a> TextParser<'a> {
    pub const fn new(text: &'a str) -> TextParser {
        TextParser { text }
    }
}

impl<'a> Parser for TextParser<'a> {
    type Output = &'a str;

    fn parse<'b>(&'b self, input: Input<'b>) -> InputAndData<'b, Self::Output> {
        let slice = &input.source[input.index..];

        if slice.starts_with(self.text) {
            (
                input.with_index(input.index + self.text.len()),
                Ok(self.text),
            )
        } else {
            (
                input,
                Err(format!(
                    "Expected '{}'; found '{}'.",
                    self.text,
                    &slice[0..10.min(slice.len())]
                )),
            )
        }
    }
}

#[macro_export]
macro_rules! text {
    ($x:literal) => {
        $crate::TextParser::new($x)
    };
}

#[macro_export]
macro_rules! literal {
    ($x:literal) => {
        $crate::TextParser::new(concat!($x)).with_value($x)
    };
}

#[test]
fn create_parser() {
    let parser = TextParser::new("Hello");

    assert_eq!(parser, TextParser { text: "Hello" });
}

#[test]
fn successful_parse_at_start() {
    let parser = TextParser::new("Hello");
    let input = Input::new("Hello world!");
    let (input, data) = parser.parse(input);

    assert_eq!(
        input,
        Input {
            source: "Hello world!",
            index: 5
        }
    );

    assert_eq!(data, Ok("Hello"));
}

#[test]
fn failed_parse_at_start() {
    let parser = TextParser::new("Hello");
    let input = Input::new("Hi world!");
    let (input, data) = parser.parse(input);

    assert_eq!(
        input,
        Input {
            source: "Hi world!",
            index: 0
        }
    );

    assert_eq!(data, Err("Expected 'Hello'; found 'Hi world!'.".to_owned()));
}
