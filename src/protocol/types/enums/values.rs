use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MessageType {
    Sync = 0x0,
    Delay_Req = 0x1,
    Pdelay_Req = 0x2,
    Pdelay_Resp = 0x3,
    Follow_Up = 0x8,
    Delay_Resp = 0x9,
    Pdelay_Resp_Follow_Up = 0xA,
    Announce = 0xB,
    Signaling = 0xC,
    Management = 0xD,
}
impl_try_from_Int!(u8, MessageType);

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TimeSource {
    ATOMIC_CLOCK = 0x10,
    GPS = 0x20,
    TERRESTRIAL_RADIO = 0x30,
    PTP = 0x40,
    NTP = 0x50,
    HAND_SET = 0x60,
    OTHER = 0x90,
    INTERNAL_OSCILLATOR = 0xA0,
}
impl_try_from_Int!(u8, TimeSource);

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ClockAcurracy {
    Unknown = 0xFF,
}
impl_try_from_Int!(u8, ClockAcurracy);
