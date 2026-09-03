use encode::combinators::Cond;
use encode::Encodable;

fn main() {
    let mut buf = Vec::new();
    Cond::new(5u16, |_: &u16| true).encode(&mut buf).unwrap();
}
