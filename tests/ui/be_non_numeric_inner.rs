use encode::combinators::BE;
use encode::Encodable;

fn main() {
    let mut buf = Vec::new();
    BE::new("hello").encode(&mut buf).unwrap();
}
