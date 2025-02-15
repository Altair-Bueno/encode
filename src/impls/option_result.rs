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
    use crate::encoders::InsufficientSpace;
    use crate::Encodable;

    #[test]
    fn assert_that_some_option_can_be_encoded() {
        let mut encoder = &mut [0u8; 32] as &mut [u8];
        Some(42u8).encode(&mut encoder).unwrap();
    }

    #[test]
    fn assert_that_none_encodes_nothing() {
        let mut encoder = &mut [0u8; 32] as &mut [u8];
        let option: Option<u8> = None;
        option.encode(&mut encoder).unwrap();
    }

    #[test]
    fn assert_that_err_result_returns_the_error() {
        let mut encoder = &mut [0u8; 32] as &mut [u8];
        assert!(Err::<(), _>(InsufficientSpace)
            .encode(&mut encoder)
            .is_err());
    }
}
