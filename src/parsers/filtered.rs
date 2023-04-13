use crate::{Parser, ParserState, UnknownParserState};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FilteredParser<P: Parser, F: Copy + Fn(&P::Output) -> bool> {
    base: P,
    filter: F,
}

impl<P: Parser, F: Copy + Fn(&P::Output) -> bool> FilteredParser<P, F> {
    pub fn new(base: P, filter: F) -> FilteredParser<P, F> {
        FilteredParser { base, filter }
    }
}

impl<P: Parser, F: Copy + Fn(&P::Output) -> bool> Parser for FilteredParser<P, F> {
    type Output = P::Output;

    fn parse<'a>(&self, state: &'a UnknownParserState<'a>) -> ParserState<'a, Self::Output> {
        self.base.parse(state).filter(self.filter)
    }
}
