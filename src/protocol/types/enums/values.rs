use crate::protocol::types::primitive::int::BitSerialize;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, BitSerialize)]
#[repr(u8)]
pub enum MessageType {
    Sync = 0x0u8,
    Delay_Req = 0x1u8,
    Pdelay_Req = 0x2u8,
    Pdelay_Resp = 0x3u8,
    Follow_Up = 0x8u8,
    Delay_Resp = 0x9u8,
    Pdelay_Resp_Follow_Up = 0xAu8,
    Announce = 0xBu8,
    Signaling = 0xCu8,
    Management = 0xDu8,
}
impl_try_from_Int!(u8, MessageType);

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, BitSerialize)]
#[repr(u8)]
pub enum TimeSource {
    ATOMIC_CLOCK = 0x10u8,
    GPS = 0x20u8,
    TERRESTRIAL_RADIO = 0x30u8,
    PTP = 0x40u8,
    NTP = 0x50u8,
    HAND_SET = 0x60u8,
    OTHER = 0x90u8,
    INTERNAL_OSCILLATOR = 0xA0u8,
}
impl_try_from_Int!(u8, TimeSource);

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, BitSerialize)]
#[repr(u8)]
pub enum ClockAcurracy {
    Unknown = 0xFFu8,
}
impl_try_from_Int!(u8, ClockAcurracy);
