use crate::packet;

pub struct RequestPacket
{
    // TODO remove magic_num, version, and packet_type. Make payload_length
    // and payload private fields
    pub magic_num: u16,
    pub version: u16,
    pub packet_type: u8,
    pub payload_length: u8,
    pub payload: Vec<u8>,
}

impl RequestPacket {
    pub fn new( resource_num: u16 ) -> Result<Self, packet::PacketError>
    {
        let resource_num_big: u8 = ((resource_num >> 8) & 0xFF) as u8;
        let resource_num_small: u8 = (resource_num & 0xFF) as u8;
        let payload: Vec<u8> = vec![ resource_num_big, resource_num_small ];
        let len: u8 = payload.len() as u8;

        Ok( Self {
            magic_num: packet::MAGIC_NUM,
            version: packet::PROTOCOL_VERSION,
            packet_type: packet::PACKET_ID_REQUEST,
            payload_length: len,
            payload: payload,
        })
    }

    pub fn to_vec( &self ) -> Vec<u8>
    {
        let mut out_vec: Vec<u8> = vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            self.packet_type,
            self.payload_length,
        ];
        out_vec.extend( self.payload.clone() );
        return out_vec;
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

    #[test]
    fn request_packet_deserialize()
    {
        let pack = RequestPacket::new( 0xAB12 ).unwrap();
        let pack_vec = pack.to_vec();
        assert_eq!( pack_vec, vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            packet::PACKET_ID_REQUEST,
            2,
            0xAB,
            0x12,
        ]);
    }
}
