use crate::{Input, InputAndData, Parser};

pub trait ParseAsChoice {
    type Output;

    fn parse_choice<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output>;
}

macro_rules! add_parse_as_choice_to_tuple {
    ($($x:ident),+) => {
        #[allow(non_snake_case)]
        impl< Z, $($x: Parser<Output = Z>),+  > ParseAsChoice for ( $($x),+ , ) {
            type Output = Z;

            fn parse_choice<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
                let ( $($x),+, ) = self;

                $(
                    let (next_input, $x) = $x.parse(input);

                    if $x.is_ok() {
                        return (next_input, $x);
                    }
                )+

                (input, Err("Failed to match any alternative.".to_owned()))
            }
        }
    };
}

add_parse_as_choice_to_tuple!(A);
add_parse_as_choice_to_tuple!(A, B);
add_parse_as_choice_to_tuple!(A, B, C);
add_parse_as_choice_to_tuple!(A, B, C, D);
add_parse_as_choice_to_tuple!(A, B, C, D, E);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
add_parse_as_choice_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);

pub struct TupleChoiceParser<T: ParseAsChoice> {
    parsers: T,
}

impl<T: ParseAsChoice> Parser for TupleChoiceParser<T> {
    type Output = T::Output;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
        self.parsers.parse_choice(input)
    }
}

pub fn choice<T: ParseAsChoice>(parsers: T) -> TupleChoiceParser<T> {
    TupleChoiceParser { parsers }
}

#[test]
fn hello_world() {
    use crate::TextParser;

    let parser = choice((
        TextParser::new("Hello"),
        TextParser::new(" "),
        TextParser::new("word"),
    ));

    let state = Input::new("Hello world!");
    let (input, state) = parser.parse(state);
    assert_eq!(input, Input::new("Hello world!").with_index(5));
    assert_eq!(state, Ok("Hello"));

    let state = Input::new("word Hello!");
    let (input, state) = parser.parse(state);
    assert_eq!(input, Input::new("word Hello!").with_index(4));
    assert_eq!(state, Ok("word"));
}
