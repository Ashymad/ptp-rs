use crate::protocol::types::enums::ClockAcurracy;
use crate::protocol::types::primitive::*;
use crate::protocol::types::primitive::int::{Int, BitSerialize};


#[allow(dead_code)] pub struct TimeInterval; //TODO

#[derive(Debug, PartialEq, BitSerialize)]
pub struct Timestamp {
    pub secondsField: UInteger48,
    pub nanosecondsField: UInteger32,
}

pub type ClockIdentity = [Octet; 8];

pub fn test() {
    use bitstream_io::{BigEndian, BitWriter};

    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    let ts = Timestamp{ secondsField: Int(0), nanosecondsField: Int(0) };
    ts.bit_serialize(&mut writer).unwrap();
}

#[derive(Debug, PartialEq)]
pub struct PortIdentity {
    pub clockIdentity: ClockIdentity,
    pub portNumber: UInteger16,
}

#[allow(dead_code)] pub struct PortAddress; //TODO

#[derive(Debug, PartialEq)]
pub struct ClockQuality {
    pub clockClass: UInteger8,
    pub clockAccuracy: ClockAcurracy,
    pub offsetScaledLogVariance: UInteger16,
}

#[allow(dead_code)] pub struct TLV; //TODO

#[allow(dead_code)] pub struct PTPText; //TODO

#[allow(dead_code)] pub struct FaultRecord; //TODO
