use crate::{Input, InputAndData, Parser};

pub trait ParseAsSequence {
    type Output;

    fn parse_seq<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output>;
}

macro_rules! add_parse_as_seq_to_tuple {
    ($($x:ident),+) => {
        #[allow(non_snake_case)]
        impl< $($x: Parser),+  > ParseAsSequence for ( $($x),+ , ) {
            type Output = ($($x::Output),+,);

            fn parse_seq<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
                let ( $($x),+, ) = self;

                let original_input = input;

                $(
                    let (input, $x) = $x.parse(input);

                    if $x.is_err() {
                        return (original_input, unsafe { Err($x.unwrap_err_unchecked()) });
                    }
                )+

                (input, Ok(($(unsafe { $x.unwrap_unchecked() }),+,)))
            }
        }
    };
}

add_parse_as_seq_to_tuple!(A);
add_parse_as_seq_to_tuple!(A, B);
add_parse_as_seq_to_tuple!(A, B, C);
add_parse_as_seq_to_tuple!(A, B, C, D);
add_parse_as_seq_to_tuple!(A, B, C, D, E);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
add_parse_as_seq_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);

pub struct TupleSequenceParser<T: ParseAsSequence> {
    parsers: T,
}

impl<T: ParseAsSequence> Parser for TupleSequenceParser<T> {
    type Output = T::Output;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
        self.parsers.parse_seq(input)
    }
}

pub fn seq<T: ParseAsSequence>(parsers: T) -> TupleSequenceParser<T> {
    TupleSequenceParser { parsers }
}

#[test]
fn hello_world() {
    use crate::TextParser;

    let parser = seq((
        TextParser::new("Hello"),
        TextParser::new(" "),
        TextParser::new("world"),
    ));

    let state = Input::new("Hello world!");

    let (input, state) = parser.parse(state);

    assert_eq!(input, Input::new("Hello world!").with_index(11));

    assert_eq!(state, Ok(("Hello", " ", "world")));
}
