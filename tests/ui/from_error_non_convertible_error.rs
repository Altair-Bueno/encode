use encode::combinators::FromError;
use encode::Encodable;

fn main() {
    let mut buf = Vec::new();
    FromError::<_, ()>::new("hello").encode(&mut buf).unwrap();
}
