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
    pub fn new( resource_num: u16 ) -> Result<Self, PacketError>
    {
        let resource_num_big: u8 = ((resource_num >> 8) & 0xFF) as u8;
        let resource_num_small: u8 = (resource_num & 0xFF) as u8;
        let payload: Vec<u8> = vec![ resource_num_big, resource_num_small ];
        let len: u8 = payload.len() as u8;

        Ok( Self {
            magic_num: MAGIC_NUM,
            version: PROTOCOL_VERSION,
            packet_type: PACKET_ID_REQUEST,
            payload_length: len,
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
        let pack_result = RequestPacket::new( 0xAB12 );

        assert_eq!( pack_result.is_ok(), true );

        let pack = pack_result.unwrap();
        assert_eq!( pack.payload_length, 2 );
        assert_eq!( pack.payload, vec![ 0xAB, 0x12 ] );
    }
}
