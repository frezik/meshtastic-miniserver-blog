pub mod request;

const MAGIC_NUM_BE: u8 = 0xBB;
const MAGIC_NUM_LE: u8 = 0x50;
const MAGIC_NUM: u16 = ((MAGIC_NUM_BE as u16) << 8)
    | (MAGIC_NUM_LE as u16);
const PROTOCOL_VERSION_BE: u8 = 0x00;
const PROTOCOL_VERSION_LE: u8 = 0x01;
const PROTOCOL_VERSION: u16 = ((PROTOCOL_VERSION_BE as u16) << 8)
    | (PROTOCOL_VERSION_LE as u16);

pub enum PacketType {
    Request,
}

impl PacketType {
    fn value( &self ) -> u8
    {
        match *self {
            PacketType::Request => 0x00,
        }
    }
}

#[derive(Debug)]
pub enum PacketError {
    PayloadTooLarge,
}

pub fn packet_to_vec(
    packet_type: PacketType,
    payload: Vec<u8>,
) -> Result< Vec<u8>, PacketError >
{
    let payload_len = payload.len();
    if payload_len > 255 {
        return Err( PacketError::PayloadTooLarge );
    }

    let mut out_vec: Vec<u8> = vec![
        MAGIC_NUM_BE,
        MAGIC_NUM_LE,
        PROTOCOL_VERSION_BE,
        PROTOCOL_VERSION_LE,
        packet_type.value(),
        payload_len as u8,
    ];
    out_vec.extend( payload.clone() );
    return Ok( out_vec );
}
