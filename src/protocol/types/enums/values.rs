use bitstream_io::{BitWriter, Endianness};
use std::io;
use crate::protocol::types::primitive::int::{BitSerialize, BitSerializeProto, Int};
use crate::protocol::types::primitive::int::constrain::{Constrain, u4};
use std::convert::TryFrom;

enum_Int!(MessageType<u4> {
    Sync = 0x0,
    Delay_Req = 0x1,
    Pdelay_Req = 0x2,
    Pdelay_Resp = 0x3,
    Follow_Up = 0x8,
    Delay_Resp = 0x9,
    Pdelay_Resp_Follow_Up = 0xA,
    Announce = 0xB,
    Signaling = 0xC,
    Management = 0xD
});

enum_Int!(TimeSource<u8> {
    ATOMIC_CLOCK = 0x10,
    GPS = 0x20,
    TERRESTRIAL_RADIO = 0x30,
    PTP = 0x40,
    NTP = 0x50,
    HAND_SET = 0x60,
    OTHER = 0x90,
    INTERNAL_OSCILLATOR = 0xA0
});

enum_Int!(ClockAcurracy<u8> {
    Unknown = 0xFF
});
