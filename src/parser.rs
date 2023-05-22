use crate::{Input, InputAndData};

/// Parsers should return their initial input if they fail.
pub trait Parser {
    type Output;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output>;

    fn filter<F: Fn(&Self::Output) -> bool + Copy>(self, op: F) -> FilteredParser<Self, F>
    where
        Self: Sized,
    {
        FilteredParser { base: self, op }
    }

    fn flat_map<T, F: Fn(Self::Output) -> Result<T, String> + Copy>(
        self,
        op: F,
    ) -> FlatMappedParser<Self, T, F>
    where
        Self: Sized,
    {
        FlatMappedParser { base: self, op }
    }

    fn map<T, F: Fn(Self::Output) -> T + Copy>(self, op: F) -> MappedParser<Self, T, F>
    where
        Self: Sized,
    {
        MappedParser { base: self, op }
    }

    fn with_value<T: Clone>(self, value: T) -> ConstantParser<Self, T>
    where
        Self: Sized,
    {
        ConstantParser { base: self, value }
    }
}

pub struct ConstantParser<P: Parser, T: Clone> {
    base: P,
    value: T,
}

impl<P: Parser, T: Clone> Parser for ConstantParser<P, T> {
    type Output = T;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
        let (input, data) = self.base.parse(input);

        (
            input,
            match data {
                Ok(_) => Ok(self.value.clone()),
                Err(error) => Err(error),
            },
        )
    }
}

pub struct FilteredParser<P: Parser, F: Fn(&P::Output) -> bool + Copy> {
    base: P,
    op: F,
}

impl<P: Parser, F: Fn(&P::Output) -> bool + Copy> Parser for FilteredParser<P, F> {
    type Output = P::Output;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
        let (next_input, data) = self.base.parse(input);

        match data {
            Ok(value) => {
                if (self.op)(&value) {
                    (next_input, Ok(value))
                } else {
                    (input, Err("Failed `filter` check.".to_string()))
                }
            }
            Err(error) => (next_input, Err(error)),
        }
    }
}

pub struct FlatMappedParser<P: Parser, T, F: Fn(P::Output) -> Result<T, String> + Copy> {
    base: P,
    op: F,
}

impl<P: Parser, T, F: Fn(P::Output) -> Result<T, String> + Copy> Parser
    for FlatMappedParser<P, T, F>
{
    type Output = T;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
        let (next_input, data) = self.base.parse(input);

        match data {
            Ok(value) => match (self.op)(value) {
                Ok(value) => (next_input, Ok(value)),
                Err(error) => (input, Err(error)),
            },
            Err(error) => (next_input, Err(error)),
        }
    }
}

pub struct MappedParser<P: Parser, T, F: Fn(P::Output) -> T + Copy> {
    base: P,
    op: F,
}

impl<P: Parser, T, F: Fn(P::Output) -> T + Copy> Parser for MappedParser<P, T, F> {
    type Output = T;

    fn parse<'a>(&'a self, input: Input<'a>) -> InputAndData<'a, Self::Output> {
        let (input, data) = self.base.parse(input);
        (input, data.map(self.op))
    }
}
