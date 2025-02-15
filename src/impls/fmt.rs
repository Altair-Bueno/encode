use core::fmt::Arguments;
use core::fmt::Write;

use crate::Encodable;
use crate::Encoder;

struct Adapter<'a, AEncoder, AError> {
    encoder: &'a mut AEncoder,
    error: Option<AError>,
}

impl<AEncoder: Encoder> Write for Adapter<'_, AEncoder, AEncoder::Error> {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if let Err(error) = self.encoder.put_slice(s.as_bytes()) {
            self.error = Some(error);
            Err(core::fmt::Error)
        } else {
            Ok(())
        }
    }
}

impl<E: Encoder> Encodable<E> for Arguments<'_> {
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

    #[test]
    fn assert_that_arguments_can_be_encoded() {
        format_args!("Hello, {}!", "world").encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_arguments_report_encoder_errors() {
        let mut encoder = &mut [0u8; 0] as &mut [u8];
        let result = format_args!("Hello, {}!", "world").encode(&mut encoder);
        assert_eq!(result, Err(crate::encoders::InsufficientSpace));
    }
}
