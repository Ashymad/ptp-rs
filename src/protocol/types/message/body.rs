use crate::protocol::types::derived::*;
use crate::protocol::types::enums::TimeSource;
use crate::protocol::types::primitive::int::BitSerialize;
use crate::protocol::types::primitive::*;

#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Announce {
    pub originTimestamp: Timestamp,
    pub currentUtcOffset: Integer16,
    pub _reserved: Octet,
    pub grandmasterPriority1: UInteger8,
    pub grandmasterClockQuality: ClockQuality,
    pub grandmasterPriority2: UInteger8,
    pub grandmasterIdentity: ClockIdentity,
    pub stepsRemoved: UInteger16,
    pub timeSource: TimeSource,
}

#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Sync {
    pub originTimestamp: Timestamp,
}

#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Delay_Req {
    pub originTimestamp: Timestamp,
}

#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Follow_Up {
    pub preciseOriginTimestamp: Timestamp,
}

#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Delay_Resp {
    pub receiveTimestamp: Timestamp,
    pub requestingPortIdentity: PortIdentity,
}
