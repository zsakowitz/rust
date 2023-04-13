pub type ParserData<T> = Result<T, String>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParserState<'a, T> {
    pub data: ParserData<T>,
    pub index: usize,
    pub source: &'a str,
}

impl ParserState<'_, ()> {
    pub fn new<'a>(source: &'a str) -> ParserState<'a, ()> {
        ParserState {
            source,
            index: 0,
            data: Ok(()),
        }
    }
}

impl<'a, T: Clone> ParserState<'a, T> {
    pub fn with_index(&self, new_index: usize) -> ParserState<T> {
        ParserState {
            data: self.data.clone(),
            index: new_index,
            source: self.source,
        }
    }
}

impl<'a, T> ParserState<'a, T> {
    pub fn as_unknown(&self) -> UnknownParserState {
        UnknownParserState {
            index: self.index,
            source: self.source,
        }
    }

    pub fn map<U, F: Fn(T) -> U>(self, mapper: F) -> ParserState<'a, U> {
        ParserState {
            data: self.data.map(mapper),
            index: self.index,
            source: self.source,
        }
    }

    pub fn filter<F: Fn(&T) -> bool>(self, filter: F) -> ParserState<'a, T> {
        ParserState {
            data: match self.data {
                Ok(value) if !filter(&value) => Err("Failed to match filter.".to_owned()),
                _ => self.data,
            },
            index: self.index,
            source: self.source,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnknownParserState<'a> {
    pub index: usize,
    pub source: &'a str,
}

impl UnknownParserState<'_> {
    pub fn with_index(&self, new_index: usize) -> UnknownParserState {
        UnknownParserState {
            index: new_index,
            source: self.source,
        }
    }

    pub fn as_ok<T>(&self, data: T) -> ParserState<T> {
        ParserState {
            data: Ok(data),
            index: self.index,
            source: self.source,
        }
    }

    pub fn as_err<T>(&self, error: String) -> ParserState<T> {
        ParserState {
            data: Err(error),
            index: self.index,
            source: self.source,
        }
    }
}

impl<'a, T> From<ParserState<'a, T>> for UnknownParserState<'a> {
    fn from(value: ParserState<'a, T>) -> Self {
        Self {
            index: value.index,
            source: value.source,
        }
    }
}

#[test]
fn create_parser_state() {
    let state = ParserState::new("Hello world");

    assert_eq!(
        state,
        ParserState {
            data: Ok(()),
            index: 0,
            source: "Hello world",
        }
    );
}

#[test]
fn map_parser_state() {
    let state = ParserState::new("Hello world");
    let state = state.map(|_| 23);

    assert_eq!(
        state,
        ParserState {
            data: Ok(23),
            index: 0,
            source: "Hello world",
        }
    );
}

#[test]
fn filter_parser_state() {
    let state = ParserState::new("Hello world")
        .map(|_| 23)
        .filter(|x| *x == 23);

    assert_eq!(
        state,
        ParserState {
            data: Ok(23),
            index: 0,
            source: "Hello world",
        }
    );
}

#[test]
fn filter_parser_state_2() {
    let state = ParserState::new("Hello world")
        .map(|_| 42)
        .filter(|x| *x == 23);

    assert_eq!(
        state,
        ParserState {
            data: Err("Failed to match filter.".to_owned()),
            index: 0,
            source: "Hello world",
        }
    );
}

#[test]
fn into_unknown() {
    let state = ParserState::new("Hello world");

    assert_eq!(
        UnknownParserState {
            index: 0,
            source: "Hello world",
        },
        state.into()
    );
}

#[test]
fn as_unknown() {
    let state = ParserState::new("Hello world");

    assert_eq!(
        UnknownParserState {
            index: 0,
            source: "Hello world",
        },
        state.as_unknown()
    );
}
