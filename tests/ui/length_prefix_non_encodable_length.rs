use core::num::TryFromIntError;
use encode::combinators::LengthPrefix;
use encode::Encodable;

fn main() {
    let mut buf = Vec::new();
    LengthPrefix::<_, u32, TryFromIntError>::new("hello")
        .encode(&mut buf)
        .unwrap();
}
