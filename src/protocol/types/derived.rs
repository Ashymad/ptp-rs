use crate::protocol::types::enums::ClockAcurracy;
use crate::protocol::types::primitive::*;

pub struct TimeInterval;

#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub secondsField: UInteger48,
    pub nanosecondsField: UInteger32,
}

pub type ClockIdentity<'a> = &'a [Octet; 8];

#[derive(Debug, PartialEq)]
pub struct PortIdentity<'a> {
    pub clockIdentity: ClockIdentity<'a>,
    pub portNumber: UInteger16,
}

pub struct PortAddress;

#[derive(Debug, PartialEq)]
pub struct ClockQuality {
    pub clockClass: UInteger8,
    pub clockAccuracy: ClockAcurracy,
    pub offsetScaledLogVariance: UInteger16,
}

pub struct TLV;

pub struct PTPText;

pub struct FaultRecord;
