pub trait Constrain {
    type Type;
    const BITS: u32;
    fn max() -> Self::Type;
    fn min() -> Self::Type;
}

macro_rules! impl_constrain {
    ($name:ident, U, $bits:literal, $type:ty) => {
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $name;
        impl Constrain for $name {
            type Type = $type;
            fn max() -> Self::Type {
                (2 as $type).pow($bits - 1) - 1 + (2 as $type).pow($bits - 1)
            }
            fn min() -> Self::Type {
                0
            }
            const BITS: u32 = $bits;
        }
    };
    ($name:ident, I, $bits:literal, $type:ty) => {
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $name;
        impl Constrain for $name {
            type Type = $type;
            fn max() -> Self::Type {
                (2 as $type).pow($bits - 2) - 1 + (2 as $type).pow($bits - 2)
            }
            fn min() -> Self::Type {
                -(2 as $type).pow($bits - 2) - (2 as $type).pow($bits - 2)
            }
            const BITS: u32 = $bits;
        }
    };
}

impl_constrain!(U1, U, 1, u8);
impl_constrain!(U4, U, 4, u8);
impl_constrain!(U48, U, 48, u64);

impl_constrain!(U8, U, 8, u8);
impl_constrain!(I8, I, 8, i8);
impl_constrain!(U16, U, 16, u16);
impl_constrain!(I16, I, 16, i16);
impl_constrain!(U32, U, 32, u32);
impl_constrain!(I32, I, 32, i32);
impl_constrain!(U64, U, 64, u64);
impl_constrain!(I64, I, 64, i64);
