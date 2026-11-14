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

        let result = core::fmt::write(&mut adapter, *self);
        if let Some(error) = adapter.error {
            return Err(error);
        }
        result.expect(
            "a formatting trait implementation returned an error when the underlying stream did not",
        );
        Ok(())
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

    struct FailingDisplay;

    impl core::fmt::Display for FailingDisplay {
        fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            Err(core::fmt::Error)
        }
    }

    struct ErrorSwallowingDisplay;

    impl core::fmt::Display for ErrorSwallowingDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let _ = f.write_str("does not fit");
            f.write_str("")
        }
    }

    #[test]
    #[should_panic(
        expected = "a formatting trait implementation returned an error when the underlying stream did not"
    )]
    fn assert_that_arguments_panic_when_display_impls_raise_their_own_errors() {
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        let _ = format_args!("{FailingDisplay}").encode(&mut encoder);
    }

    #[test]
    fn assert_that_arguments_report_encoder_errors_swallowed_by_display_impls() {
        let mut buf = [0u8; 3];
        let mut encoder = &mut buf as &mut [u8];
        let result = format_args!("{ErrorSwallowingDisplay}").encode(&mut encoder);
        assert!(result.is_err());
    }
}
