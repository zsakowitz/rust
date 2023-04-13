use crate::{Parser, ParserState, UnknownParserState};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ManyParser<T: Parser> {
    parser: T,
}

impl<T: Parser> ManyParser<T> {
    pub fn new(parser: T) -> ManyParser<T> {
        ManyParser { parser }
    }
}

impl<T: Parser + Copy> Copy for ManyParser<T> {}

impl<T: Parser> Parser for ManyParser<T> {
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

            if let Ok(value) = next_state.data {
                output.push(value);
            } else {
                break;
            }
        }

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
    let parser = ManyParser::new(parser);
    let state = ParserState::new("Hello world!");
    let state = &state.as_unknown();
    let state = parser.parse(state);

    assert_eq!(
        state,
        ParserState {
            data: Ok(Vec::new()),
            index: 0,
            source: "Hello world!"
        }
    )
}

#[test]
fn matching_once() {
    use super::text::TextParser;

    let parser = TextParser::new("hi ");
    let parser = ManyParser::new(parser);
    let state = ParserState::new("hi world!");
    let state = &state.as_unknown();
    let state = parser.parse(state);

    assert_eq!(
        state,
        ParserState {
            data: Ok(vec!["hi "]),
            index: 3,
            source: "hi world!"
        }
    )
}

#[test]
fn matching_four_times() {
    use super::text::TextParser;

    let parser = TextParser::new("hi ");
    let parser = ManyParser::new(parser);
    let state = ParserState::new("hi hi hi hi world!");
    let state = &state.as_unknown();
    let state = parser.parse(state);

    assert_eq!(
        state,
        ParserState {
            data: Ok(vec!["hi "; 4]),
            index: 12,
            source: "hi hi hi hi world!"
        }
    )
}
