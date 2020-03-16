#[macro_use]
pub mod int;

use int::constrain::{u1, u4, u48};
use int::Int;

#[allow(dead_code)]
pub type Boolean = Int<u1>;
pub type Enumeration4 = Int<u4>;
pub type Enumeration8 = Int<u8>;
pub type Enumeration16 = Int<u16>;
pub type UInteger4 = Int<u4>;
pub type Integer8 = Int<i8>;
pub type UInteger8 = Int<u8>;
pub type Integer16 = Int<i16>;
pub type UInteger16 = Int<u16>;
#[allow(dead_code)]
pub type Integer32 = Int<i32>;
pub type UInteger32 = Int<u32>;
pub type UInteger48 = Int<u48>;
pub type Integer64 = Int<i64>;
pub type Nibble = Int<u4>;
pub type Octet = Int<u8>;
