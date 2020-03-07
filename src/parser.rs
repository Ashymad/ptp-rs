use nom::{IResult};
use nom::number::streaming::{be_i8, be_u8, be_u16, be_i64};

#[allow(dead_code)]
pub mod types {
    pub mod primitive {
        //Boolean
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
        //UInteger48
        pub type Integer64      = i64;
        pub type Nibble         = u8;
        pub type Octet          = u8;
    }
    pub mod enums {
        use super::primitive::Enumeration4;
        use enum_repr::EnumRepr;

        #[EnumRepr(type = "Enumeration4")]
        #[allow(non_camel_case_types)]
        #[derive(FromPrimitive, Clone, Copy, Debug, Eq, PartialEq)]
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
    #[allow(non_snake_case)]
    pub mod derived {
        pub struct TimeInterval;
        pub struct Timestamp;
        pub type ClockIdentity<'a> = &'a[super::primitive::Octet];
        #[derive(Debug,PartialEq)]
        pub struct PortIdentity<'a> {
            pub clockIdentity: ClockIdentity<'a>,
            pub portNumber: super::primitive::UInteger16,
        }
        pub struct PortAddress;
        pub struct ClockQuality;
        pub struct TLV;
        pub struct PTPText;
        pub struct FaultRecord;
    }
}

use types::primitive::*;
use types::enums::MessageType;
use types::derived::{PortIdentity};

#[allow(non_snake_case)]
#[derive(Debug,PartialEq)]
pub struct PtpHeader<'a> {
    pub transportSpecific: Nibble,
    pub messageType: MessageType,
    _reserved1: Nibble,
    pub versionPTP: UInteger4,
    pub messageLength: UInteger16,
    pub domainNumber: UInteger8,
    _reserved2: Octet,
    pub flagField: &'a[Octet],
    pub correctionField: Integer64,
    _reserved3: &'a[Octet],
    pub sourcePortIdentity: PortIdentity<'a>,
    pub sequenceId: UInteger16,
    pub controlField: UInteger8,
    pub logMessageInterval: Integer8,
}

#[allow(non_snake_case)]
pub fn parse_port_identity(data: &[u8]) -> IResult<&[u8],PortIdentity> {
    do_parse!(
        data,
        clockIdentity: take!(8) >>
        portNumber: be_u16 >>
        ( PortIdentity{clockIdentity, portNumber} )
    )
}


named!(#[allow(non_snake_case)], pub parse_ptp_header<PtpHeader>,
    do_parse!(
        b0: bits!(
            tuple!(take_bits!(4u8), take_bits!(4u8), take_bits!(4u8), take_bits!(4u8))
        ) >>
        messageLength: be_u16 >>
        domainNumber: be_u8 >>
        _reserved2: take!(1) >>
        flagField: take!(2) >>
        correctionField: be_i64 >>
        _reserved3: take!(4) >>
        sourcePortIdentity: parse_port_identity >>
        sequenceId: be_u16 >>
        controlField: be_u8 >>
        logMessageInterval: be_i8 >>
        (
            PtpHeader {
                transportSpecific: b0.0,
                messageType: num::FromPrimitive::from_u8(b0.1).unwrap(),
                _reserved1: b0.2,
                versionPTP: b0.3,
                messageLength ,
                domainNumber,
                _reserved2: _reserved2[0],
                flagField,
                correctionField,
                _reserved3,
                sourcePortIdentity,
                sequenceId,
                controlField,
                logMessageInterval,
            }
        )
    )
);





