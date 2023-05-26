use regex::Regex;

use crate::Parser;

pub struct RegexParser {
    regex: Regex,
}

impl RegexParser {
    pub fn new(regex: Regex) -> Self {
        RegexParser { regex }
    }

    pub fn regex(&self) -> &'_ Regex {
        &self.regex
    }
}

impl Parser for RegexParser {
    type Output = String;

    fn parse<'a>(&'a self, input: crate::Input<'a>) -> crate::InputAndData<'a, Self::Output> {
        let result = self.regex.find_at(input.source, input.index);

        match result {
            Some(value) => (input.with_index(value.end()), Ok(value.as_str().to_owned())),
            None => {
                let slice = &input.source[input.index..];

                (
                    input,
                    Err(format!(
                        "Expected '{}'; found '{}'.",
                        self.regex.as_str(),
                        &slice[0..10.min(slice.len())]
                    )),
                )
            }
        }
    }
}

#[macro_export]
macro_rules! regex {
    ($x:literal) => {
        $crate::RegexParser::new(::regex::Regex::new($x).unwrap())
    };
}

#[test]
fn match_decimal_digit() {
    use crate::Input;

    let parser = regex!(r"\d");

    let state = Input::new("42 worlds");
    let (input, state) = parser.parse(state);
    assert_eq!(input, Input::new("42 worlds").with_index(1));
    assert_eq!(state, Ok("4".to_string()));
}
