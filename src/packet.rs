pub mod request;
use rand;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

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

    let mut rng = SmallRng::from_rng( &mut rand::rng() );
    let connection_id: Vec<u8> = vec![
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
    ];

    let mut out_vec: Vec<u8> = vec![
        MAGIC_NUM_BE,
        MAGIC_NUM_LE,
        PROTOCOL_VERSION_BE,
        PROTOCOL_VERSION_LE,
    ];
    out_vec.extend( connection_id );
    out_vec.push( packet_type.value() );
    out_vec.push( payload_len as u8 );
    out_vec.extend( payload.clone() );

    return Ok( out_vec );
}
