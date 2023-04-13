use crate::{FilteredParser, MappedParser, ParserState, UnknownParserState};

pub trait Parser {
    type Output;

    fn parse<'a>(&self, state: &'a UnknownParserState<'a>) -> ParserState<'a, Self::Output>;

    fn map<T, F>(self, mapper: F) -> MappedParser<Self, T, F>
    where
        Self: Sized,
        F: Copy + Fn(Self::Output) -> T,
    {
        MappedParser::new(self, mapper)
    }

    fn filter<F>(self, filter: F) -> FilteredParser<Self, F>
    where
        Self: Sized,
        F: Copy + Fn(&Self::Output) -> bool,
    {
        FilteredParser::new(self, filter)
    }
}
