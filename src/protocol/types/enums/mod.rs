use crate::protocol::types::primitive::{Enumeration16, Enumeration4, Enumeration8};

use std::convert::TryFrom;

#[allow(non_camel_case_types)]
pub mod values;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Enumeration<E, T> {
    Enum(T),
    Unknown(E),
}

impl<U: Into<E> + PartialOrd + Copy, E, T: TryFrom<E>> From<U> for Enumeration<E, T> {
    fn from(val: U) -> Enumeration<E, T> {
        match T::try_from(val.into()) {
            Ok(en) => Enumeration::Enum(en),
            Err(_) => Enumeration::Unknown(val.into()),
        }
    }
}

macro_rules! impl_from_enumeration {
    ($E:ty) => {
        impl<T: Into<$E>> From<Enumeration<$E, T>> for $E {
            fn from(val: Enumeration<$E, T>) -> $E {
                match val {
                    Enumeration::Enum(en) => Into::<$E>::into(en),
                    Enumeration::Unknown(un) => un,
                }
            }
        }
    };
}

impl_from_enumeration!(Enumeration4);
impl_from_enumeration!(Enumeration8);
impl_from_enumeration!(Enumeration16);

impl<
        E: Copy
            + PartialOrd
            + std::ops::Add
            + From<<E as std::ops::Add>::Output>
            + From<Enumeration<E, T>>,
        T: TryFrom<E> + Into<E> + Copy,
    > std::ops::AddAssign for Enumeration<E, T>
{
    fn add_assign(&mut self, other: Self) {
        *self = Self::from(E::from(E::from(other) + E::from(*self)))
    }
}

impl<
        E: Copy
            + PartialOrd
            + std::ops::Shl<U>
            + From<<E as std::ops::Shl<U>>::Output>
            + From<Enumeration<E, T>>,
        T: TryFrom<E> + Into<E>,
        U,
    > std::ops::Shl<U> for Enumeration<E, T>
{
    type Output = Self;

    fn shl(self, other: U) -> Self {
        Self::from(E::from(E::from(self) << other))
    }
}

impl<
        E: Copy
            + PartialOrd
            + std::ops::Shr<U>
            + From<<E as std::ops::Shr<U>>::Output>
            + From<Enumeration<E, T>>,
        T: TryFrom<E> + Into<E>,
        U,
    > std::ops::Shr<U> for Enumeration<E, T>
{
    type Output = Self;

    fn shr(self, other: U) -> Self {
        Self::from(E::from(E::from(self) >> other))
    }
}

pub type MessageType = Enumeration<Enumeration4, values::MessageType>;
pub type ClockAcurracy = Enumeration<Enumeration8, values::ClockAcurracy>;
pub type TimeSource = Enumeration<Enumeration8, values::TimeSource>;
