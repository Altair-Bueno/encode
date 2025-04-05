use crate::Encodable;
use crate::Encoder;

impl<T, E> Encodable<E> for Option<T>
where
    T: Encodable<E>,
    E: Encoder,
{
    type Error = T::Error;

    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Some(value) => value.encode(encoder),
            None => Ok(()),
        }
    }
}

impl<T, E> Encodable<E> for Result<T, T::Error>
where
    T: Encodable<E>,
    E: Encoder,
    T::Error: Clone,
{
    type Error = T::Error;

    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            Ok(value) => value.encode(encoder),
            Err(err) => Err(err.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoders::InsufficientSpace;

    const BUF_SIZE: usize = 64;

    #[test]
    fn assert_that_some_option_can_be_encoded() {
        let expected = b"\x01";
        let encodable = Some(1u8);

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_none_encodes_nothing() {
        let expected = b"";
        let encodable = None::<u8>;

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_ok_result_can_be_encoded() {
        let expected = b"\x01";
        let encodable = Ok(1u8);

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_err_result_returns_the_error() {
        let encodable = Err::<u8, _>(InsufficientSpace);

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap_err();
    }
}
