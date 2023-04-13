use crate::{Parser, ParserState, UnknownParserState};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Many1Parser<T: Parser> {
    parser: T,
}

impl<T: Parser> Many1Parser<T> {
    pub fn new(parser: T) -> Many1Parser<T> {
        Many1Parser { parser }
    }
}

impl<T: Parser + Copy> Copy for Many1Parser<T> {}

impl<T: Parser> Parser for Many1Parser<T> {
    type Output = Vec<<T as Parser>::Output>;

    fn parse<'a>(
        &self,
        UnknownParserState { mut index, source }: &'a UnknownParserState<'a>,
    ) -> ParserState<'a, Self::Output> {
        let mut output = vec![];

        loop {
            let state = UnknownParserState { index, source };
            let next_state = self.parser.parse(&state);

            index = next_state.index;

            match next_state.data {
                Ok(value) => output.push(value),
                Err(error) if output.len() == 0 => {
                    return ParserState {
                        data: Err(error),
                        index,
                        source,
                    }
                }
                Err(_) => break,
            }
        }

        assert!(output.len() != 0);

        ParserState {
            data: Ok(output),
            index,
            source,
        }
    }
}

#[test]
fn matching_zero_times() {
    use super::text::TextParser;

    let parser = TextParser::new("hi ");
    let parser = Many1Parser::new(parser);
    let state = ParserState::new("Hello world!");
    let state = &state.as_unknown();
    let state = parser.parse(state);

    assert_eq!(
        state,
        ParserState {
            data: Err("Expected 'hi '; found 'Hello worl'.".to_owned()),
            index: 0,
            source: "Hello world!",
        }
    )
}

#[test]
fn matching_once() {
    use super::text::TextParser;

    let parser = TextParser::new("hi ");
    let parser = Many1Parser::new(parser);
    let state = ParserState::new("hi world!");
    let state = &state.as_unknown();
    let state = parser.parse(state);

    assert_eq!(
        state,
        ParserState {
            data: Ok(vec!["hi "]),
            index: 3,
            source: "hi world!",
        }
    )
}

#[test]
fn matching_four_times() {
    use super::text::TextParser;

    let parser = TextParser::new("hi ");
    let parser = Many1Parser::new(parser);
    let state = ParserState::new("hi hi hi hi world!");
    let state = &state.as_unknown();
    let state = parser.parse(state);

    assert_eq!(
        state,
        ParserState {
            data: Ok(vec!["hi "; 4]),
            index: 12,
            source: "hi hi hi hi world!",
        }
    )
}
