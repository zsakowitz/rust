pub type ParserData<T> = Result<T, String>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParserState<T> {
    pub data: ParserData<T>,
    pub index: usize,
    pub source: String,
}

impl ParserState<()> {
    fn new(source: String) -> ParserState<()> {
        ParserState {
            source,
            index: 0,
            data: Ok(()),
        }
    }
}

impl<T> ParserState<T> {
    pub fn with_index(&self, new_index: usize) -> ParserState<T> {
        ParserState {
            data: self.data,
            index: new_index,
            source: self.source,
        }
    }

    pub fn as_unknown(&self) -> UnknownParserState {
        UnknownParserState {
            index: self.index,
            source: self.source,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnknownParserState {
    pub index: usize,
    pub source: String,
}

impl UnknownParserState {
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

impl<T> From<ParserState<T>> for UnknownParserState {
    fn from(value: ParserState<T>) -> Self {
        Self {
            index: value.index,
            source: value.source,
        }
    }
}

#[test]
fn create() {
    let state = ParserState::new("Hello world".to_owned());

    assert_eq!(
        state,
        ParserState {
            data: Ok(()),
            index: 0,
            source: "Hello world".to_owned(),
        }
    )
}

#[test]
fn into() {
    let state = ParserState::new("Hello world".to_owned());

    assert_eq!(
        UnknownParserState {
            index: 0,
            source: "Hello world".to_owned(),
        },
        state.into()
    )
}
