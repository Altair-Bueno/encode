use core::fmt::Arguments;
use core::fmt::Write;

use crate::Encodable;
use crate::StrEncoder;

struct Adapter<'a, AEncoder, AError> {
    encoder: &'a mut AEncoder,
    error: Option<AError>,
}

impl<AEncoder: StrEncoder> Write for Adapter<'_, AEncoder, AEncoder::Error> {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match s.encode(self.encoder) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.error = Some(error);
                Err(core::fmt::Error)
            }
        }
    }
}

impl<E: StrEncoder> Encodable<E> for Arguments<'_> {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let mut adapter = Adapter {
            encoder,
            error: None,
        };

        if core::fmt::write(&mut adapter, *self).is_ok() {
            Ok(())
        } else {
            Err(adapter.error.expect("Adapter always sets error on failure"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUF_SIZE: usize = 64;

    #[test]
    fn assert_that_arguments_can_be_encoded() {
        let expected = b"Hello world, I present you with a number: 42";

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        format_args!("Hello {}, I present you with a number: {}", "world", 42)
            .encode(&mut encoder)
            .unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_arguments_report_encoder_errors() {
        let mut buf = [0u8; 0];
        let mut encoder = &mut buf as &mut [u8];
        format_args!("Hello {}, I present you with a number: {}", "world", 42)
            .encode(&mut encoder)
            .unwrap_err();
    }
}
