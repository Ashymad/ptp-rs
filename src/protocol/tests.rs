#[test]
pub fn test_bit_serialize() {
    use bitstream_io::{BigEndian, BitWriter};
    use crate::protocol::types::primitive::int::{Int, BitSerialize};
    use crate::protocol::types::derived::{Timestamp, PortIdentity};

    let mut writer = BitWriter::endian(Vec::new(), BigEndian);
    let ts = Timestamp{ secondsField: Int(234), nanosecondsField: Int(10928) };
    let pi = PortIdentity{ clockIdentity: [Int(15); 8], portNumber: Int(09898) };
    ts.bit_serialize(&mut writer).unwrap();
    pi.bit_serialize(&mut writer).unwrap();
    let wwriter = &writer.into_writer();
    let pts = crate::protocol::parser::parse_timestamp(wwriter).unwrap();
    let ppi = crate::protocol::parser::parse_port_identity(pts.0).unwrap();
    assert_eq!(ppi.1, pi);
    assert_eq!(pts.1, ts);
}
