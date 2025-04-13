#![allow(unused_variables)]
use crate::BaseEncoder;
use crate::Encodable;

// https://users.rust-lang.org/t/macro-to-impl-trait-for-tuple/79165/3
macro_rules! impl_encodable_for_tuple {
    ($($T:tt)*) => {
        paste::paste! {
            impl<ENC, A, $($T,)*> Encodable<ENC> for (A,$($T,)*)
            where
                ENC: BaseEncoder,
                A: Encodable<ENC>,
                $($T: Encodable<ENC, Error = A::Error>,)*
            {
                type Error = A::Error;
                #[inline]
                fn encode(&self, encoder:&mut ENC) -> Result<(), Self::Error> {
                    let (a, $([<$T:lower>],)*) = self;
                    a.encode(encoder)?;
                    $([<$T:lower>].encode(encoder)?;)*
                    Ok(())
                }

            }
        }
    };
}

impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W X Y);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W X);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V W);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U V);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T U);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S T);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R S);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q R);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P Q);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O P);
impl_encodable_for_tuple!(B C D E F G H I J K L M N O);
impl_encodable_for_tuple!(B C D E F G H I J K L M N);
impl_encodable_for_tuple!(B C D E F G H I J K L M);
impl_encodable_for_tuple!(B C D E F G H I J K L);
impl_encodable_for_tuple!(B C D E F G H I J K);
impl_encodable_for_tuple!(B C D E F G H I J);
impl_encodable_for_tuple!(B C D E F G H I);
impl_encodable_for_tuple!(B C D E F G H);
impl_encodable_for_tuple!(B C D E F G);
impl_encodable_for_tuple!(B C D E F);
impl_encodable_for_tuple!(B C D E);
impl_encodable_for_tuple!(B C D);
impl_encodable_for_tuple!(B C);
impl_encodable_for_tuple!(B);
impl_encodable_for_tuple!();

impl<E: BaseEncoder> Encodable<E> for () {
    type Error = E::Error;

    #[inline]
    fn encode(&self, _encoder: &mut E) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn assert_that_tuples_can_be_encoded() {
        let encodable = ("hello", "world");
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_unit_can_be_encoded() {
        let encodable = ();
        encodable.encode(&mut ()).unwrap();
    }
}
