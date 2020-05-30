#[test]
pub fn test_bit_serialize() {
    use crate::protocol::types::derived::{PortIdentity, Timestamp};
    use crate::protocol::types::primitive::int::{BitSerialize, Int};
    use bitstream_io::{BigEndian, BitWriter};

    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    let ts = Timestamp {
        secondsField: Int::new(23),
        nanosecondsField: Int::new(19),
    };
    let pi = PortIdentity {
        clockIdentity: [Int::new(15); 8],
        portNumber: Int::new(98),
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
            transportSpecific: Int::new(0),
            messageType: Enum(MessageType::Announce),
            _reserved1: Int::new(0),
            versionPTP: Int::new(2),
            messageLength: Int::new(64),
            domainNumber: Int::new(0),
            _reserved2: Int::new(0),
            flagField: [Int::new(0), Int::new(0)],
            correctionField: Int::new(0),
            _reserved3: [Int::new(0), Int::new(0), Int::new(0), Int::new(0)],
            sourcePortIdentity: PortIdentity {
                clockIdentity: [
                    Int::new(184),
                    Int::new(39),
                    Int::new(235),
                    Int::new(255),
                    Int::new(254),
                    Int::new(146),
                    Int::new(177),
                    Int::new(166),
                ],
                portNumber: Int::new(1),
            },
            sequenceId: Int::new(13123),
            controlField: Int::new(5),
            logMessageInterval: Int::new(1),
        },
        body: Body::Announce(body::Announce {
            originTimestamp: Timestamp {
                secondsField: Int::new(0),
                nanosecondsField: Int::new(0),
            },
            currentUtcOffset: Int::new(37),
            _reserved: Int::new(0),
            grandmasterPriority1: Int::new(128),
            grandmasterClockQuality: ClockQuality {
                clockClass: Int::new(248),
                clockAccuracy: Unknown(Int::new(254)),
                offsetScaledLogVariance: Int::new(65535),
            },
            grandmasterPriority2: Int::new(128),
            grandmasterIdentity: [
                Int::new(184),
                Int::new(39),
                Int::new(235),
                Int::new(255),
                Int::new(254),
                Int::new(146),
                Int::new(177),
                Int::new(166),
            ],
            stepsRemoved: Int::new(0),
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
