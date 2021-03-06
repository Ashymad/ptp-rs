use crate::protocol::types::derived::*;
use crate::protocol::types::enums::MessageType;
use crate::protocol::types::primitive::int::BitSerialize;
use crate::protocol::types::primitive::*;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod body;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Header {
    pub transportSpecific: Nibble,
    pub messageType: MessageType,
    pub _reserved1: Nibble,
    pub versionPTP: UInteger4,
    pub messageLength: UInteger16,
    pub domainNumber: UInteger8,
    pub _reserved2: Octet,
    pub flagField: [Octet; 2],
    pub correctionField: Integer64,
    pub _reserved3: [Octet; 4],
    pub sourcePortIdentity: PortIdentity,
    pub sequenceId: UInteger16,
    pub controlField: UInteger8,
    pub logMessageInterval: Integer8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub enum Body {
    Announce(body::Announce),
    Sync(body::Sync),
    Delay_Req(body::Delay_Req),
    Follow_Up(body::Follow_Up),
    Delay_Resp(body::Delay_Resp),
    Empty,
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, BitSerialize, Clone, Copy)]
pub struct Message {
    pub header: Header,
    pub body: Body,
}
