pub mod primitive {
    pub type Boolean        = bool;
    pub type Enumeration4   = u8;
    pub type Enumeration8   = u8;
    pub type Enumeration16  = u8;
    pub type UInteger4      = u8;
    pub type Integer8       = i8;
    pub type UInteger8      = u8;
    pub type Integer16      = i16;
    pub type UInteger16     = u16;
    pub type Integer32      = i32;
    pub type UInteger32     = u32;
    pub type UInteger48     = u64;
    pub type Integer64      = i64;
    pub type Nibble         = u8;
    pub type Octet          = u8;
}

pub mod enums {
    use super::primitive::{Enumeration4, Enumeration8};
    use std::convert::TryFrom;

    #[allow(non_camel_case_types)]
    pub mod values {
        use num_enum::{TryFromPrimitive, IntoPrimitive};

        #[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
        #[repr(u8)]
        pub enum MessageType {
            Sync                  = 0x0,
            Delay_Req             = 0x1,
            Pdelay_Req            = 0x2,
            Pdelay_Resp           = 0x3,
            Follow_Up             = 0x8,
            Delay_Resp            = 0x9,
            Pdelay_Resp_Follow_Up = 0xA,
            Announce              = 0xB,
            Signaling             = 0xC,
            Management            = 0xD,
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Enumeration<E, T> {
        Enum(T),
        Unknown(E)
    }
    
    impl<E: Copy, T: TryFrom<E>> From<E> for Enumeration<E, T> {
        fn from(val: E) -> Enumeration<E, T> {
            match T::try_from(val) {
                Ok(en) => Enumeration::Enum(en),
                Err(_) => Enumeration::Unknown(val)
            }
        }
    }

    impl<E: Copy + std::ops::Add + From<<E as std::ops::Add>::Output>, T: TryFrom<E> + Into<E> + Copy> std::ops::AddAssign for Enumeration<E, T> {
        fn add_assign(&mut self, other: Self) {
            let rhs: E = match other {
                Enumeration::Enum(en) => en.into(),
                Enumeration::Unknown(un) => un,
            };
            *self = Self::from(E::from(rhs + match self {
                Enumeration::Enum(en) => Into::<E>::into(*en),
                Enumeration::Unknown(un) => *un
            }))
        }
    }

    impl<E: Copy + std::ops::Shl<U> + From<<E as std::ops::Shl<U>>::Output>, T: TryFrom<E> + Into<E>, U> std::ops::Shl<U> for Enumeration<E, T> {
        type Output = Self;

        fn shl(self, other: U) -> Self {
            Self::from(E::from(match self {
                Enumeration::Enum(en) => Into::<E>::into(en),
                Enumeration::Unknown(un) => un
            } << other))
        }
    }

    impl<E: Copy + std::ops::Shr<U> + From<<E as std::ops::Shr<U>>::Output>, T: TryFrom<E> + Into<E>, U> std::ops::Shr<U> for Enumeration<E, T> {
        type Output = Self;

        fn shr(self, other: U) -> Self {
            Self::from(E::from(match self {
                Enumeration::Enum(en) => Into::<E>::into(en),
                Enumeration::Unknown(un) => un
            } >> other))
        }
    }

    pub type MessageType = Enumeration<Enumeration4, values::MessageType>;


    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct ClockAcurracy(pub Enumeration8);

    #[allow(non_upper_case_globals)]
    impl ClockAcurracy {
        pub const Unknown : ClockAcurracy = ClockAcurracy(0xFF);
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct TimeSource(pub Enumeration8);

    impl TimeSource {
        pub const ATOMIC_CLOCK        : TimeSource = TimeSource(0x10);
        pub const GPS                 : TimeSource = TimeSource(0x20);
        pub const TERRESTRIAL_RADIO   : TimeSource = TimeSource(0x30);
        pub const PTP                 : TimeSource = TimeSource(0x40);
        pub const NTP                 : TimeSource = TimeSource(0x50);
        pub const HAND_SET            : TimeSource = TimeSource(0x60);
        pub const OTHER               : TimeSource = TimeSource(0x90);
        pub const INTERNAL_OSCILLATOR : TimeSource = TimeSource(0xA0);
    }
}

#[allow(non_snake_case)]
pub mod derived {
    use super::primitive::*;
    use super::enums::ClockAcurracy;
    
    pub struct TimeInterval;
    #[derive(Debug,PartialEq)]
    pub struct Timestamp {
        pub secondsField: UInteger48,
        pub nanosecondsField: UInteger32
    }
    pub type ClockIdentity<'a> = &'a[Octet; 8];
    #[derive(Debug,PartialEq)]
    pub struct PortIdentity<'a> {
        pub clockIdentity: ClockIdentity<'a>,
        pub portNumber: UInteger16,
    }
    pub struct PortAddress;
    #[derive(Debug,PartialEq)]
    pub struct ClockQuality {
       pub clockClass: UInteger8,
       pub clockAccuracy: ClockAcurracy,
       pub offsetScaledLogVariance: UInteger16
    }
    pub struct TLV;
    pub struct PTPText;
    pub struct FaultRecord;
}

use primitive::*;
use derived::*;
use enums::MessageType;


#[allow(non_snake_case)]
#[derive(Debug,PartialEq)]
pub struct Header<'a> {
    pub transportSpecific: Nibble,
    pub messageType: MessageType,
    pub _reserved1: Nibble,
    pub versionPTP: UInteger4,
    pub messageLength: UInteger16,
    pub domainNumber: UInteger8,
    pub _reserved2: Octet,
    pub flagField: &'a[Octet],
    pub correctionField: Integer64,
    pub _reserved3: &'a[Octet],
    pub sourcePortIdentity: PortIdentity<'a>,
    pub sequenceId: UInteger16,
    pub controlField: UInteger8,
    pub logMessageInterval: Integer8,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod body {
    use super::primitive::*;
    use super::derived::*;
    use super::enums::TimeSource;

    #[derive(Debug,PartialEq)]
    pub struct Announce<'a> {
        pub originTimestamp: Timestamp,
        pub currentUtcOffset: Integer16,
        pub _reserved: Octet,
        pub grandmasterPriority1: UInteger8,
        pub grandmasterClockQuality: ClockQuality,
        pub grandmasterPriority2: UInteger8,
        pub grandmasterIdentity: ClockIdentity<'a>,
        pub stepsRemoved: UInteger16,
        pub timeSource: TimeSource
    }

    #[derive(Debug,PartialEq)]
    pub struct Sync {
        pub originTimestamp: Timestamp
    }

    #[derive(Debug,PartialEq)]
    pub struct Delay_Req {
        pub originTimestamp: Timestamp
    }

    #[derive(Debug,PartialEq)]
    pub struct Follow_Up {
        pub preciseOriginTimestamp: Timestamp
    }

    #[derive(Debug,PartialEq)]
    pub struct Delay_Resp<'a> {
        pub receiveTimestamp: Timestamp,
        pub requestingPortIdentity: PortIdentity<'a>
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug,PartialEq)]
pub enum Body<'a> {
    Announce(body::Announce<'a>),
    Sync(body::Sync),
    Delay_Req(body::Delay_Req),
    Follow_Up(body::Follow_Up),
    Delay_Resp(body::Delay_Resp<'a>), 
    Empty
}

#[allow(non_snake_case)]
#[derive(Debug,PartialEq)]
pub struct Message<'a> {
    pub header: Header<'a>,
    pub body: Body<'a>
}
