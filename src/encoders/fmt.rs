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
