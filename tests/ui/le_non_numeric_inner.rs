use encode::combinators::LE;
use encode::Encodable;

fn main() {
    let mut buf = Vec::new();
    LE::new("hello").encode(&mut buf).unwrap();
}
