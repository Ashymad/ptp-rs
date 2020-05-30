use bitstream_io::{BitWriter, Endianness, Numeric};
use std::fmt;
use std::io;
use std::ops::{Add, AddAssign, Shl, Shr};

#[allow(non_camel_case_types)]
pub mod constrain;
use constrain::Constrain;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Int<C: Constrain>(C::Type);

impl<C: Constrain> Int<C> {
    pub fn new(c: C::Type) -> Self where C::Type: PartialOrd {
        if c > C::max() {
            panic!("Value overflows!")
        } else if c < C::min() {
            panic!("Value underflows!")
        } else {
            return Int(c)
        }
    }
    pub fn into_inner(self) -> C::Type {
        return self.0
    }
}

impl<C: Constrain> fmt::Debug for Int<C>
where
    C::Type: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{:?}{}",
            self.0,
            std::any::type_name::<C>().rsplitn(2, "::").next().unwrap()
        )
    }
}

impl<C: Constrain> Constrain for Int<C> {
    type Type = C::Type;
    fn max() -> Self::Type {
        C::max()
    }
    fn min() -> Self::Type {
        C::min()
    }
    const BITS: u32 = C::BITS;
}

// Eq required only so it doesn't conflict
impl<C: Constrain, U: Eq> From<U> for Int<C>
where
    C::Type: From<U> + fmt::Debug + PartialOrd,
{
    fn from(other: U) -> Int<C> {
        let oth = other.into();
        debug_assert!(
            oth <= C::max() && oth >= C::min(),
            "Conversion from {:?} resulted in overflow!",
            oth
        );
        Int(oth)
    }
}

pub trait BitSerializeProto<W: io::Write, E: Endianness>
where
    Self: Numeric,
{
    fn bit_serialize_proto(self, bits: u32, bw: &mut BitWriter<W, E>) -> Result<(), io::Error>;
}
pub trait BitSerialize<W: io::Write, E: Endianness> {
    fn bit_serialize(self, bw: &mut BitWriter<W, E>) -> Result<(), io::Error>;
}

macro_rules! impl_bit_serialize_proto {
    ($i:ident: $($type:ty),*) => {
        $(
            impl<W: io::Write, E: Endianness> BitSerializeProto<W, E> for $type {
                fn bit_serialize_proto(self, bits: u32, bw: &mut BitWriter<W, E>) -> Result<(), io::Error> {
                    impl_bit_serialize_proto!(@write $i, bw, bits, self)
                }
            }
            impl<W: io::Write, E: Endianness> BitSerialize<W, E> for $type {
                fn bit_serialize(self, bw: &mut BitWriter<W, E>) -> Result<(), io::Error> {
                    self.bit_serialize_proto(Self::bits_size(), bw)
                }
            }
        )*
    };
    (@write u, $bw:ident, $b:ident, $s:ident) => {
        $bw.write($b, $s)
    };
    (@write i, $bw: ident, $b:ident, $s:ident) => {
        $bw.write_signed($b, $s)
    };
}

impl_bit_serialize_proto!(i: i8, i16, i32, i64);
impl_bit_serialize_proto!(u: u8, u16, u32, u64);

impl<W: io::Write, E: Endianness, C: Constrain> BitSerialize<W, E> for Int<C>
where
    C::Type: BitSerializeProto<W, E>,
{
    fn bit_serialize(self, bw: &mut BitWriter<W, E>) -> Result<(), io::Error> {
        self.0.bit_serialize_proto(C::BITS, bw)
    }
}

macro_rules! impl_bit_serialize {
    ([_; $L:literal]) => {
        impl<W: io::Write, E: Endianness, U: BitSerialize<W, E> + Copy> BitSerialize<W, E>
            for [U; $L]
        {
            fn bit_serialize(self, bw: &mut BitWriter<W, E>) -> Result<(), io::Error> {
                for idx in 0..$L {
                    self[idx].bit_serialize(bw)?
                }
                Ok(())
            }
        }
    };
}

impl_bit_serialize!([_; 2]);
impl_bit_serialize!([_; 4]);
impl_bit_serialize!([_; 8]);

pub trait TryFrom<T> {
    type Error;
    fn try_from(t: T) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

impl<T: TryFrom<U>, U> TryInto<T> for U {
    type Error = <T as TryFrom<U>>::Error;
    fn try_into(self: Self) -> Result<T, Self::Error> {
        T::try_from(self)
    }
}

impl<C: Constrain, U> TryFrom<U> for Int<C>
where
    C::Type: From<U> + PartialOrd,
{
    type Error = &'static str;
    fn try_from(other: U) -> Result<Self, Self::Error> {
        let oth = other.into();
        if oth <= C::max() && oth >= C::min() {
            Ok(Int(oth))
        } else {
            Err("Conversion to Int resulted in overflow!")
        }
    }
}

macro_rules! impl_try_from {
    ([_; $L: literal]) => {
        impl<'a, U: Copy + TryInto<Int<C>>, C: Constrain> TryFrom<&'a [U]> for [Int<C>; $L]
        where
            C::Type: Default,
            Int<C>: Copy,
        {
            type Error = &'static str;
            fn try_from(other: &'a [U]) -> Result<Self, Self::Error> {
                if other.len() == $L {
                    let mut out = [Int(C::Type::default()); $L];
                    for idx in 0..$L {
                        match other[idx].try_into() {
                            Ok(val) => out[idx] = val,
                            Err(_) => return Err("Couldn't convert to Int"),
                        }
                    }
                    Ok(out)
                } else {
                    Err("Incompatible length of slice")
                }
            }
        }
    };
}

impl_try_from!([_; 2]);
impl_try_from!([_; 4]);
impl_try_from!([_; 8]);

impl<C: Constrain> Add for Int<C>
where
    C::Type: Add + From<<C::Type as Add>::Output> + fmt::Debug + PartialOrd,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let sum = (self.0 + other.0).into();
        debug_assert!(
            sum <= C::max() && sum >= C::min(),
            "Sum {:?} resulted in overflow!",
            sum
        );
        Int(sum)
    }
}

impl<C: Constrain> AddAssign for Int<C>
where
    C::Type: AddAssign + fmt::Debug + PartialOrd,
{
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        debug_assert!(
            self.0 <= C::max() && self.0 >= C::min(),
            "Sum {:?} resulted in overflow!",
            self.0
        );
    }
}

impl<C: Constrain, U> Shl<U> for Int<C>
where
    C::Type: Shl<U> + From<<C::Type as Shl<U>>::Output> + fmt::Debug + PartialOrd,
{
    type Output = Self;
    fn shl(self, other: U) -> Self::Output {
        let out = (self.0 << other).into();
        debug_assert!(out <= C::max() && out >= C::min());
        Int(out)
    }
}

impl<C: Constrain, U> Shr<U> for Int<C>
where
    C::Type: Shr<U> + From<<C::Type as Shr<U>>::Output> + fmt::Debug + PartialOrd,
{
    type Output = Self;
    fn shr(self, other: U) -> Self::Output {
        let out = (self.0 >> other).into();
        debug_assert!(out <= C::max() && out >= C::min());
        Int(out)
    }
}

macro_rules! enum_Int {
    ($name:ident<$type:ty> {$($field:ident = $value:expr),*}) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $field = $value
            ),*
        }
        impl Into<Int<$type>> for $name {
            fn into(self) -> Int<$type> {
                Int::new(match self {
                    $(
                        $name::$field => $value
                    ),*
                })
            }
        }
        impl TryFrom<Int<$type>> for $name {
            type Error = &'static str;

            fn try_from(other: Int<$type>) -> Result<Self, Self::Error> {
                match other.into_inner() {
                    $(
                        $value => Ok($name::$field)
                    ),*,
                        _ => Err("Value doesn't have corresponding variant")
                }
            }
        }
       impl<W: io::Write, E: Endianness> BitSerialize<W, E> for $name {
            fn bit_serialize(self, bw: &mut BitWriter<W, E>) -> Result<(), io::Error> {
                let val: <$type as Constrain>::Type = match self {
                    $(
                        $name::$field => $value
                    ),*
                };
                val.bit_serialize_proto(<$type as Constrain>::BITS, bw)
            }
        }
    }
}

