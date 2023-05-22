use crate::{literal, text, InputAndData, Parser, TextParser};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct EitherParser<A: Parser, B: Parser> {
    left: A,
    right: B,
}

impl<A: Parser, B: Parser> EitherParser<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<A: Parser, B: Parser> Parser for EitherParser<A, B> {
    type Output = Either<A::Output, B::Output>;

    fn parse<'a>(&'a self, input: crate::Input<'a>) -> InputAndData<'a, Self::Output> {
        let (left_input, left_data) = self.left.parse(input);

        if let Ok(left_data) = left_data {
            return (left_input, Ok(Either::Left(left_data)));
        }

        let (right_input, right_data) = self.right.parse(input);

        match right_data {
            Ok(right_data) => (right_input, Ok(Either::Right(right_data))),
            Err(error) => (input, Err(error)),
        }
    }
}

#[test]
fn string_and_number() {
    let left = text!("Hello");
    let right = literal!(42);
    let parser = EitherParser::new(left, right);
}
