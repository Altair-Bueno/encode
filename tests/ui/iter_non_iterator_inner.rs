use encode::combinators::Iter;
use encode::Encodable;

fn main() {
    let mut buf = Vec::new();
    Iter::new("hello").encode(&mut buf).unwrap();
}
