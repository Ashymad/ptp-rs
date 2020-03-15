pub trait Constrain {
    type Type;
    const BITS: u32;
    fn max() -> Self::Type;
    fn min() -> Self::Type;
}

macro_rules! impl_constrain {
    ($name:ident, u:$bits:literal, $type:ty) => {
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $name;
        impl Constrain for $name {
            type Type = $type;
            fn max() -> Self::Type {
                (2 as $type).wrapping_pow($bits).wrapping_sub(1)
            }
            fn min() -> Self::Type {
                0
            }
            const BITS: u32 = $bits;
        }
    };
    ($name:ident, i:$bits:literal, $type:ty) => {
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $name;
        impl Constrain for $name {
            type Type = $type;
            fn max() -> Self::Type {
                (2 as $type).wrapping_pow($bits - 1).wrapping_sub(1)
            }
            fn min() -> Self::Type {
                (2 as $type).wrapping_pow($bits - 1).wrapping_neg()
            }
            const BITS: u32 = $bits;
        }
    };
}

impl_constrain!(U1, u:1, u8);
impl_constrain!(U4, u:4, u8);
impl_constrain!(U48, u:48, u64);

impl_constrain!(U8, u:8, u8);
impl_constrain!(I8, i:8, i8);
impl_constrain!(U16, u:16, u16);
impl_constrain!(I16, i:16, i16);
impl_constrain!(U32, u:32, u32);
impl_constrain!(I32, i:32, i32);
impl_constrain!(U64, u:64, u64);
impl_constrain!(I64, i:64, i64);
