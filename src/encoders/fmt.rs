use core::fmt::Error;
use core::fmt::Formatter;

use crate::BaseEncoder;
use crate::StrEncoder;

impl BaseEncoder for Formatter<'_> {
    type Error = Error;
}

impl StrEncoder for Formatter<'_> {
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error> {
        self.write_str(string)
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;
    use crate::Encodable;
    use alloc::string::String;
    use core::fmt::Write;

    #[test]
    fn assert_that_formatter_can_be_used_as_an_encoder() {
        struct MyEncodable;

        impl<E> Encodable<E> for MyEncodable
        where
            E: StrEncoder,
        {
            type Error = E::Error;
            fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
                "Hello world".encode(encoder)
            }
        }

        impl core::fmt::Display for MyEncodable {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
                self.encode(f)
            }
        }
        let mut output = String::new();
        writeln!(&mut output, "{}", MyEncodable).expect("Failed to write to output");

        assert_eq!(output, "Hello world\n");
    }
}
