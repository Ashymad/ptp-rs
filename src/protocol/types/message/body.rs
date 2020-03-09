use crate::protocol::types::derived::*;
use crate::protocol::types::enums::TimeSource;
use crate::protocol::types::primitive::*;

#[derive(Debug, PartialEq)]
pub struct Announce<'a> {
    pub originTimestamp: Timestamp,
    pub currentUtcOffset: Integer16,
    pub _reserved: Octet,
    pub grandmasterPriority1: UInteger8,
    pub grandmasterClockQuality: ClockQuality,
    pub grandmasterPriority2: UInteger8,
    pub grandmasterIdentity: ClockIdentity<'a>,
    pub stepsRemoved: UInteger16,
    pub timeSource: TimeSource,
}

#[derive(Debug, PartialEq)]
pub struct Sync {
    pub originTimestamp: Timestamp,
}

#[derive(Debug, PartialEq)]
pub struct Delay_Req {
    pub originTimestamp: Timestamp,
}

#[derive(Debug, PartialEq)]
pub struct Follow_Up {
    pub preciseOriginTimestamp: Timestamp,
}

#[derive(Debug, PartialEq)]
pub struct Delay_Resp<'a> {
    pub receiveTimestamp: Timestamp,
    pub requestingPortIdentity: PortIdentity<'a>,
}
