pub mod article_response;
pub mod directory_response;
pub mod error_response;
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
    DirectoryResponse,
    ArticleResponse,
    ErrorResponse,
}

impl PacketType {
    fn value( &self ) -> u8
    {
        match *self {
            PacketType::Request => 0x00,
            PacketType::DirectoryResponse => 0x01,
            PacketType::ArticleResponse => 0x02,
            PacketType::ErrorResponse => 0x03,
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
    connection_id: u16,
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
        ( ((connection_id >> 8) & 0xFF) as u8 ),
        ( (connection_id & 0xFF) as u8 ),
    ];
    out_vec.push( payload_len as u8 );
    out_vec.extend( payload.clone() );

    return Ok( out_vec );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_oversized_payload()
    {
        let mut payload: Vec<u8> = (0..255).collect();
        payload.push( 0x00 );
        let result = packet_to_vec( PacketType::Request, payload, 0x0000 );

        assert_eq!( result.is_ok(), false );
    }
}
