extern crate input_reader_trait;

#[macro_use]
extern crate input_reader_derive;

use std::str::FromStr;
use input_reader_trait::FromTokenStream;

struct Zhenyok {
    a: i32,
}

impl<'a, I: Iterator<Item=&'a String>> FromTokenStream<'a, I> for Zhenyok {
    fn parse_from(self: &mut Zhenyok, iter: &mut I) {
        self.a = iter.next().as_ref().unwrap().parse().unwrap();
    }
}

#[derive(FromTokenStream)]
struct ZhenyokDerived {
    a: i32,
    b: f32,
    c: i64
}

#[test]
fn test_from_token_stream() {
    let mut z = Zhenyok { a: 0 };

    z.parse_from(&mut vec!["1".to_string()].iter());
    assert_eq!(z.a, 1);
}

#[test]
fn test_from_token_stream_derived() {
    let mut z = ZhenyokDerived { a: 0, b: 0.0, c: 0 };

    z.parse_from(&mut vec!["1".to_string(), "2.0".to_string(), "1000".to_string()].iter());
    assert_eq!(z.a, 1);
    assert_eq!(z.b, 2.0);
    assert_eq!(z.c, 1000);
}
