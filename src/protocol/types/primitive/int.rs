use std::ops::{Add, AddAssign, Shl, Shr};

pub mod constrain {
    use std::ops::{Add, Shl, Shr};

    pub trait Constrain {
        type Type: PartialOrd
            + std::fmt::Debug
            + Copy
            + Add
            + From<<Self::Type as Add>::Output>
            + Shl<usize>
            + From<<Self::Type as Shl<usize>>::Output>
            + Shr<usize>
            + From<<Self::Type as Shr<usize>>::Output>;
        const MAX: Self::Type;
        const MIN: Self::Type;
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct U4;
    impl Constrain for U4 {
        type Type = u8;
        const MAX: Self::Type = 0xF;
        const MIN: Self::Type = 0x0;
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct U48;
    impl Constrain for U48 {
        type Type = u64;
        const MAX: Self::Type = 0xFFFFFFFFFFFF;
        const MIN: Self::Type = 0x000000000000;
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    pub struct U1;
    impl Constrain for U1 {
        type Type = u8;
        const MAX: Self::Type = 1;
        const MIN: Self::Type = 0;
    }

    macro_rules! impl_default_constrain {
        ($I:ident, $T:ty) => {
            #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
            pub struct $I;
            impl Constrain for $I {
                type Type = $T;
                const MAX: Self::Type = Self::Type::max_value();
                const MIN: Self::Type = Self::Type::min_value();
            }
        };
    }

    impl_default_constrain!(U8, u8);
    impl_default_constrain!(I8, i8);
    impl_default_constrain!(U16, u16);
    impl_default_constrain!(I16, i16);
    impl_default_constrain!(U32, u32);
    impl_default_constrain!(I32, i32);
    impl_default_constrain!(U64, u64);
    impl_default_constrain!(I64, i64);
}

use constrain::Constrain;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Int<C: Constrain>(pub C::Type);

impl<C: Constrain> std::fmt::Debug for Int<C> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:?}{}", self.0, std::any::type_name::<C>().rsplitn(2, "::").next().unwrap())
    }
}

// Eq required only so it doesn't conflict
impl<C: Constrain, U: Eq> From<U> for Int<C>
where
    C::Type: From<U>,
{
    fn from(other: U) -> Int<C> {
        let oth = other.into();
        debug_assert!(
            oth <= C::MAX && oth >= C::MIN,
            "Conversion from {:?} resulted in overflow!",
            oth
        );
        Int(oth)
    }
}

pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

macro_rules! impl_try_into_int {
    ($L: literal) => {
        impl<'a, U: Into<C::Type> + Copy, C: Constrain> TryInto<[Int<C>; $L]> for &'a [U]
        where
            C::Type: Default,
            Int<C>: Copy,
        {
            type Error = &'static str;
            fn try_into(self: Self) -> Result<[Int<C>; $L], Self::Error> {
                if self.len() == $L {
                    let mut out = [Int(C::Type::default()); $L];
                    for idx in 0..$L {
                        out[idx].0 = self[idx].into();
                    }
                    Ok(out)
                } else {
                    Err("Incompatible length of slice")
                }
            }
        }
    };
}

impl_try_into_int!(2);
impl_try_into_int!(4);
impl_try_into_int!(8);

impl<C: Constrain> Add for Int<C> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let sum = From::<<C::Type as Add>::Output>::from(self.0 + other.0);
        debug_assert!(
            sum <= C::MAX && sum >= C::MIN,
            "Sum {:?} resulted in overflow!",
            sum
        );
        Int(sum)
    }
}

impl<C: Constrain> AddAssign for Int<C> {
    fn add_assign(&mut self, other: Self) {
        let sum: C::Type = From::<<C::Type as Add>::Output>::from(self.0 + other.0);
        debug_assert!(
            sum <= C::MAX && sum >= C::MIN,
            "Sum {:?} resulted in overflow!",
            sum
        );
        self.0 = sum;
    }
}

impl<C: Constrain> Shl<usize> for Int<C> {
    type Output = Self;
    fn shl(self, other: usize) -> Self::Output {
        let out = From::<<C::Type as Shl<usize>>::Output>::from(self.0 << other);
        debug_assert!(out <= C::MAX && out >= C::MIN);
        Int(out)
    }
}

impl<C: Constrain> Shr<usize> for Int<C> {
    type Output = Self;
    fn shr(self, other: usize) -> Self::Output {
        let out = From::<<C::Type as Shr<usize>>::Output>::from(self.0 >> other);
        debug_assert!(out <= C::MAX && out >= C::MIN);
        Int(out)
    }
}

macro_rules! impl_try_from_Int {
    ($T:ty, $E:ty) => {
        impl<C: $crate::protocol::types::primitive::int::constrain::Constrain>
            std::convert::TryFrom<$crate::protocol::types::primitive::int::Int<C>> for $E
        where
            C::Type: Into<$T>,
        {
            type Error = <$E as std::convert::TryFrom<$T>>::Error;

            fn try_from(
                other: $crate::protocol::types::primitive::int::Int<C>,
            ) -> Result<Self, Self::Error> {
                <$E>::try_from(other.0.into())
            }
        }
    };
}
