#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Input<'a> {
    pub source: &'a str,
    pub index: usize,
}

pub type Data<T> = Result<T, String>;

pub type InputAndData<'a, T> = (Input<'a>, Data<T>);

impl<'a> Input<'a> {
    pub const fn new(source: &'a str) -> Input<'a> {
        Input { source, index: 0 }
    }

    pub const fn with_index(self, index: usize) -> Input<'a> {
        Input {
            source: self.source,
            index,
        }
    }
}
