use crate::packet;
use rand;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;


pub struct RequestPacket
{
    resource_num: u16,
}

impl RequestPacket {
    pub fn new( resource_num: u16 ) -> Result<Self, packet::PacketError>
    {
        Ok( Self {
            resource_num: resource_num,
        })
    }

    pub fn to_vec( &self ) -> Vec<u8>
    {
        let resource_num_big: u8 = ((self.resource_num >> 8) & 0xFF) as u8;
        let resource_num_small: u8 = (self.resource_num & 0xFF) as u8;

        let mut rng = SmallRng::from_rng( &mut rand::rng() );
        let connection_id: u16 = rng.random();

        let payload: Vec<u8> = vec![ resource_num_big, resource_num_small ];
        packet::packet_to_vec(
            packet::PacketType::Request,
            payload,
            connection_id,
        ).unwrap()
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
    }

    #[test]
    fn request_packet_deserialize()
    {
        let pack = RequestPacket::new( 0xAB12 ).unwrap();
        let pack_vec = pack.to_vec();

        let mut expect_vec: Vec<u8> = vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            packet::PacketType::Request.value(),
            // Connection ID
            pack_vec[5],
            pack_vec[6],
        ];
        expect_vec.extend( vec![
            2,
            0xAB,
            0x12,
        ].clone() );

        assert_eq!( pack_vec, expect_vec );
    }
}
