#[macro_use]
pub mod int;

use int::constrain::{U4, U8, U16, U32, U48, I8, I16, I32, I64};
use int::Int;

pub type Boolean = bool;
pub type Enumeration4 = Int<U4>;
pub type Enumeration8 = Int<U8>;
pub type Enumeration16 = Int<U16>;
pub type UInteger4 = Int<U4>;
pub type Integer8 = Int<I8>;
pub type UInteger8 = Int<U8>;
pub type Integer16 = Int<I16>;
pub type UInteger16 = Int<U16>;
pub type Integer32 = Int<I32>;
pub type UInteger32 = Int<U32>;
pub type UInteger48 = Int<U48>;
pub type Integer64 = Int<I64>;
pub type Nibble = Int<U4>;
pub type Octet = u8;
