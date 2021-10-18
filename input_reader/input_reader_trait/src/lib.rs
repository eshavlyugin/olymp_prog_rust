use std::iter::Iterator;

pub trait FromTokenStream<'a, I: Iterator<Item=&'a String>> {
    fn parse_from(self: &mut Self, iter: &mut I);
}
