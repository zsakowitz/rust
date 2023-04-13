use crate::parser_state::{ParserState, UnknownParserState};

pub trait Parser {
    type Output;

    fn parse(&self, state: &UnknownParserState) -> ParserState<Self::Output>;
}
