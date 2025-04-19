const MAGIC_NUM: u16 = 0xBB50;
const PROTOCOL_VERSION: u16 = 0x0001;
const PACKET_ID_REQUEST: u8 = 0x00;


#[derive(Debug)]
pub enum PacketError {
    PayloadTooLarge,
}

pub struct RequestPacket
{
    pub magic_num: u16,
    pub version: u16,
    pub packet_type: u8,
    pub payload_length: u8,
    pub payload: Vec<u8>,
}

impl RequestPacket {
    pub fn new( payload: Vec<u8> ) -> Result<Self, PacketError>
    {
        let len = payload.len();
        if len > 255 {
            return Err( PacketError::PayloadTooLarge );
        }

        let u8_len = len as u8;

        Ok( Self {
            magic_num: MAGIC_NUM,
            version: PROTOCOL_VERSION,
            packet_type: PACKET_ID_REQUEST,
            payload_length: u8_len,
            payload: payload,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_packet()
    {
        let pack_result = RequestPacket::new( vec![ 0, 0, 0, 0 ] );

        assert_eq!( pack_result.is_ok(), true );

        let pack = pack_result.unwrap();
        assert_eq!( pack.payload_length, 4 );
    }

    #[test]
    fn request_packet_payload_too_large()
    {
        let mut large_payload: Vec<u8> = (0..255).collect();
        large_payload.push( 0 );
        let pack_result = RequestPacket::new( large_payload );

        assert_eq!( pack_result.is_ok(), false );
    }
}
