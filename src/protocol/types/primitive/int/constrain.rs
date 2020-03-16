pub trait Constrain {
    type Type;
    const BITS: u32;
    fn max() -> Self::Type;
    fn min() -> Self::Type;
}

macro_rules! impl_constrain {
    ($sign:ident: $($type:ty),*) => {
        $(
        impl_constrain!(@impl $type, $sign, 8*std::mem::size_of::<$type>() as u32, $type);
        )*
    };
    ($name:ident, $sign:ident, $bits:expr, $type:ty) => {
        impl_constrain!(@struct $name);
        impl_constrain!(@impl $name, $sign, $bits, $type);

    };
    (@struct $name:ident) => {
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $name;
    };
    (@impl $name:ty, $sign:ident, $bits:expr, $type:ty) => {
        impl Constrain for $name {
            type Type = $type;
            impl_constrain!(@maxmin $sign, $type);
            const BITS: u32 = $bits;
        }
    };
    (@maxmin i, $type:ty) => {
        fn max() -> Self::Type {
            (2 as $type).wrapping_pow(Self::BITS - 1).wrapping_sub(1)
        }
        fn min() -> Self::Type {
            (2 as $type).wrapping_pow(Self::BITS - 1).wrapping_neg()
        }
    };
    (@maxmin u, $type:ty) => {
        fn max() -> Self::Type {
            (2 as $type).wrapping_pow(Self::BITS).wrapping_sub(1)
        }
        fn min() -> Self::Type {
            0
        }
    }
}
impl_constrain!(u1, u, 1, u8);
impl_constrain!(u4, u, 4, u8);
impl_constrain!(u48, u, 48, u64);

impl_constrain!(u: u8, u16, u32, u64);
impl_constrain!(i: i8, i16, i32, i64);
