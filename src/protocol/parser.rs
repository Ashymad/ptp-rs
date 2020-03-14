use crate::protocol::types::derived::*;
use crate::protocol::types::enums::*;
use crate::protocol::types::message::{body, Body, Header, Message};
use crate::protocol::types::primitive::int::TryInto;

use nom::error::ParseError;
use nom::number::streaming::{be_i16, be_i64, be_i8, be_u16, be_u32, be_u8};
use nom::{Err, IResult, Needed};

#[inline]
pub fn be_u48<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], u64, E> {
    if i.len() < 6 {
        Err(Err::Incomplete(Needed::Size(6)))
    } else {
        let res = ((i[0] as u64) << 40)
            + ((i[1] as u64) << 32)
            + ((i[2] as u64) << 24)
            + ((i[3] as u64) << 16)
            + ((i[4] as u64) << 8)
            + i[5] as u64;
        Ok((&i[6..], res))
    }
}

named!(#[allow(non_snake_case)], pub parse_timestamp<Timestamp>,
    do_parse!(
        secondsField: be_u48 >>
        nanosecondsField: be_u32 >>
        (
            Timestamp {
                secondsField: secondsField.into(),
                nanosecondsField: nanosecondsField.into()
            }
        )
    )
);

named!(#[allow(non_snake_case)], pub parse_port_identity<PortIdentity>,
    do_parse!(
        clockIdentity: take!(8) >>
        portNumber: be_u16 >>
        (
            PortIdentity {
                clockIdentity: clockIdentity.try_into().expect("Wrong size clockIdentity array!"),
                portNumber: portNumber.into()
            }
        )
    )
);

named!(#[allow(non_snake_case)], pub parse_clock_quality<ClockQuality>,
    do_parse!(
        clockClass: be_u8 >>
        clockAccuracy: be_u8 >>
        offsetScaledLogVariance: be_u16 >>
        (
            ClockQuality {
                clockClass: clockClass.into(),
                clockAccuracy: clockAccuracy.into(),
                offsetScaledLogVariance: offsetScaledLogVariance.into()
            }
        )
    )
);
#[allow(non_snake_case)]
pub fn parse_ptp_header<'a>(i: &'a [u8]) -> IResult<&'a [u8], Header> {
    do_parse!(
        i,
        b0: bits!(tuple!(
            take_bits!(4u8),
            take_bits!(4u8),
            take_bits!(4u8),
            take_bits!(4u8)
        )) >> messageLength: be_u16
            >> domainNumber: be_u8
            >> _reserved2: take!(1)
            >> flagField: take!(2)
            >> correctionField: be_i64
            >> _reserved3: take!(4)
            >> sourcePortIdentity: parse_port_identity
            >> sequenceId: be_u16
            >> controlField: be_u8
            >> logMessageInterval: be_i8
            >> (Header {
                transportSpecific: b0.0,
                messageType: b0.1,
                _reserved1: b0.2,
                versionPTP: b0.3,
                messageLength: messageLength.into(),
                domainNumber: domainNumber.into(),
                _reserved2: _reserved2[0].into(),
                flagField: flagField.try_into().unwrap(),
                correctionField: correctionField.into(),
                _reserved3: _reserved3.try_into().unwrap(),
                sourcePortIdentity,
                sequenceId: sequenceId.into(),
                controlField: controlField.into(),
                logMessageInterval: logMessageInterval.into(),
            })
    )
}

#[allow(non_snake_case)]
macro_rules! parse_ptp_body (
    ($i:expr, $message_type:expr) => (
        {
            match $message_type {
                Enumeration::Enum(values::MessageType::Sync) => {
                    do_parse!($i,
                        originTimestamp: parse_timestamp >>
                        (
                            Body::Sync(body::Sync {
                                originTimestamp,
                            })
                        )
                    )
                },
                Enumeration::Enum(values::MessageType::Follow_Up) => {
                    do_parse!($i,
                        preciseOriginTimestamp: parse_timestamp >>
                        (
                            Body::Follow_Up(body::Follow_Up {
                                preciseOriginTimestamp,
                            })
                        )
                    )
                },
                Enumeration::Enum(values::MessageType::Delay_Req) => {
                    do_parse!($i,
                        originTimestamp: parse_timestamp >>
                        (
                            Body::Delay_Req(body::Delay_Req {
                                originTimestamp,
                            })
                        )
                    )
                },
                Enumeration::Enum(values::MessageType::Delay_Resp) => {
                    do_parse!($i,
                        receiveTimestamp: parse_timestamp >>
                        requestingPortIdentity: parse_port_identity >>
                        (
                            Body::Delay_Resp(body::Delay_Resp {
                                receiveTimestamp,
                                requestingPortIdentity
                            })
                        )
                    )
                },
                Enumeration::Enum(values::MessageType::Announce) => {
                    do_parse!($i,
                        originTimestamp: parse_timestamp >>
                        currentUtcOffset: be_i16 >>
                        _reserved: take!(1) >>
                        grandmasterPriority1: be_u8 >>
                        grandmasterClockQuality: parse_clock_quality >>
                        grandmasterPriority2: be_u8 >>
                        grandmasterIdentity: take!(8) >>
                        stepsRemoved: be_u16 >>
                        timeSource: be_u8 >>
                        (
                            Body::Announce(body::Announce {
                                originTimestamp,
                                currentUtcOffset: currentUtcOffset.into(),
                                _reserved: _reserved[0].into(),
                                grandmasterPriority1: grandmasterPriority1.into(),
                                grandmasterClockQuality,
                                grandmasterPriority2: grandmasterPriority2.into(),
                                grandmasterIdentity: grandmasterIdentity.try_into().unwrap(),
                                stepsRemoved: stepsRemoved.into(),
                                timeSource: timeSource.into()
                            })
                        )
                    )
                },
                _ => {
                    eprintln!("Unknown message type: {:?}, couldn't parse!", $message_type);
                    Ok(($i, Body::Empty))
                }
            }
        }
    );
);

named!(#[allow(non_snake_case)], pub parse_ptp_message<Message>,
    do_parse!(
        header: parse_ptp_header >>
        body: parse_ptp_body!(header.messageType) >>
        (
            Message {
                header,
                body,
            }
        )
    )
);
