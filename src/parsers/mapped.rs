use crate::{Parser, ParserState, UnknownParserState};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MappedParser<P: Parser, T, F: Copy + Fn(P::Output) -> T> {
    base: P,
    mapper: F,
}

impl<P: Parser, T, F: Copy + Fn(P::Output) -> T> MappedParser<P, T, F> {
    pub fn new(base: P, mapper: F) -> MappedParser<P, T, F> {
        MappedParser { base, mapper }
    }
}

impl<P: Parser, T, F: Copy + Fn(P::Output) -> T> Parser for MappedParser<P, T, F> {
    type Output = T;

    fn parse<'a>(&self, state: &'a UnknownParserState<'a>) -> ParserState<'a, Self::Output> {
        self.base.parse(state).map(self.mapper)
    }
}
