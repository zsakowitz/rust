use crate::{Parser, ParserState, UnknownParserState};

impl<T: Parser> Parser for (T,) {
    type Output = (T::Output,);

    fn parse<'a>(&self, state: &'a UnknownParserState<'a>) -> ParserState<'a, Self::Output> {
        self.0.parse(state).map(|x| (x,))
    }
}

impl<A: Parser, B: Parser> Parser for (A, B) {
    type Output = (A::Output, B::Output);

    fn parse<'a>(&self, state: &'a UnknownParserState<'a>) -> ParserState<'a, Self::Output> {
        let next = self.0.parse(state);

        match next.data {
            Ok(a) => {
                let binding = UnknownParserState {
                    index: next.index,
                    source: state.source,
                };

                let next = self.1.parse(&binding);

                match next.data {
                    Ok(b) => ParserState {
                        data: Ok((a, b)),
                        index: next.index,
                        source: state.source,
                    },
                    Err(error) => state.as_err(error),
                }
            }
            Err(error) => state.as_err(error),
        }
    }
}
