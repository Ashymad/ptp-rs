#[test]
pub fn test_bit_serialize() {
    use crate::protocol::types::derived::{PortIdentity, Timestamp};
    use crate::protocol::types::primitive::int::{BitSerialize, Int};
    use bitstream_io::{BigEndian, BitWriter};

    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    let ts = Timestamp {
        secondsField: Int(23),
        nanosecondsField: Int(19),
    };
    let pi = PortIdentity {
        clockIdentity: [Int(15); 8],
        portNumber: Int(98),
    };
    ts.bit_serialize(&mut writer).unwrap();
    pi.bit_serialize(&mut writer).unwrap();
    let wwriter = &writer.into_writer();
    let pts = crate::protocol::parser::parse_timestamp(wwriter).unwrap();
    let ppi = crate::protocol::parser::parse_port_identity(pts.0).unwrap();
    assert_eq!(ppi.1, pi);
    assert_eq!(pts.1, ts);
}

#[test]
pub fn test_bit_serialize2() {
    use crate::protocol::parser::parse_ptp_message;
    use crate::protocol::types::derived::{ClockQuality, PortIdentity, Timestamp};
    use crate::protocol::types::enums::values::{MessageType, TimeSource};
    use crate::protocol::types::enums::Enumeration::{Enum, Unknown};
    use crate::protocol::types::message::{body, Body, Header, Message};
    use crate::protocol::types::primitive::int::{BitSerialize, Int};

    use bitstream_io::{BigEndian, BitWriter};
    let msg = Message {
        header: Header {
            transportSpecific: Int(0),
            messageType: Enum(MessageType::Announce),
            _reserved1: Int(0),
            versionPTP: Int(2),
            messageLength: Int(64),
            domainNumber: Int(0),
            _reserved2: Int(0),
            flagField: [Int(0), Int(0)],
            correctionField: Int(0),
            _reserved3: [Int(0), Int(0), Int(0), Int(0)],
            sourcePortIdentity: PortIdentity {
                clockIdentity: [
                    Int(184),
                    Int(39),
                    Int(235),
                    Int(255),
                    Int(254),
                    Int(146),
                    Int(177),
                    Int(166),
                ],
                portNumber: Int(1),
            },
            sequenceId: Int(13123),
            controlField: Int(5),
            logMessageInterval: Int(1),
        },
        body: Body::Announce(body::Announce {
            originTimestamp: Timestamp {
                secondsField: Int(0),
                nanosecondsField: Int(0),
            },
            currentUtcOffset: Int(37),
            _reserved: Int(0),
            grandmasterPriority1: Int(128),
            grandmasterClockQuality: ClockQuality {
                clockClass: Int(248),
                clockAccuracy: Unknown(Int(254)),
                offsetScaledLogVariance: Int(65535),
            },
            grandmasterPriority2: Int(128),
            grandmasterIdentity: [
                Int(184),
                Int(39),
                Int(235),
                Int(255),
                Int(254),
                Int(146),
                Int(177),
                Int(166),
            ],
            stepsRemoved: Int(0),
            timeSource: Enum(TimeSource::INTERNAL_OSCILLATOR),
        }),
    };
    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    msg.bit_serialize(&mut writer).unwrap();
    let wwriter = writer.into_writer();
    let msg2 = parse_ptp_message(&wwriter).unwrap();
    assert_eq!(msg2.0.len(), 0);
    assert_eq!(msg, msg2.1);
}
