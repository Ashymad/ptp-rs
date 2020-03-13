use std::convert::TryFrom;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct int<C: Constrain>(<C as Constrain>::Type);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct U4;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct U48;

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

impl Constrain for U4 {
    type Type = u8;
    const MAX: Self::Type = 0xF;
    const MIN: Self::Type = 0x0;
}

impl Constrain for U48 {
    type Type = u64;
    const MAX: Self::Type = 0xFFFFFFFFFFFF;
    const MIN: Self::Type = 0x000000000000;
}

// Eq required only so it doesn't conflict
impl<C: Constrain, U: Eq> From<U> for int<C>
where
    C::Type: From<U>,
{
    fn from(other: U) -> int<C> {
        let oth = other.into();
        debug_assert!(
            oth <= C::MAX && oth >= C::MIN,
            "Conversion from {:?} resulted in overflow!",
            oth
        );
        int(oth)
    }
}

use std::ops::{Add, AddAssign, Shl, Shr};
impl<C: Constrain> Add for int<C> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let sum = From::<<C::Type as Add>::Output>::from(self.0 + other.0);
        debug_assert!(
            sum <= C::MAX && sum >= C::MIN,
            "Sum {:?} resulted in overflow!",
            sum
        );
        int(sum)
    }
}

impl<C: Constrain> AddAssign for int<C> {
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

impl<C: Constrain> Shl<usize> for int<C> {
    type Output = Self;
    fn shl(self, other: usize) -> Self::Output {
        let out = From::<<C::Type as Shl<usize>>::Output>::from(self.0 << other);
        debug_assert!(out <= C::MAX && out >= C::MIN);
        int(out)
    }
}

impl<C: Constrain> Shr<usize> for int<C> {
    type Output = Self;
    fn shr(self, other: usize) -> Self::Output {
        let out = From::<<C::Type as Shr<usize>>::Output>::from(self.0 >> other);
        debug_assert!(out <= C::MAX && out >= C::MIN);
        int(out)
    }
}

use crate::protocol::types::enums::values;
impl<C: Constrain> TryFrom<int<C>> for values::MessageType
where
    C::Type: Into<u8>,
{
    type Error = <values::MessageType as TryFrom<u8>>::Error;

    fn try_from(other: int<C>) -> Result<Self, Self::Error> {
        values::MessageType::try_from(other.0.into())
    }
}

pub type Boolean = bool;
pub type Enumeration4 = int<U4>;
pub type Enumeration8 = u8;
pub type Enumeration16 = u16;
pub type UInteger4 = int<U4>;
pub type Integer8 = i8;
pub type UInteger8 = u8;
pub type Integer16 = i16;
pub type UInteger16 = u16;
pub type Integer32 = i32;
pub type UInteger32 = u32;
pub type UInteger48 = int<U48>;
pub type Integer64 = i64;
pub type Nibble = int<U4>;
pub type Octet = u8;
