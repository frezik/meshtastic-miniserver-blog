use crate::packet;
use rand;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;


pub struct RequestPacket
{
    /// The ID of the directory entry to request
    resource_num: u16,
    /// The connection ID. This isn't set if we're constructing a fresh 
    /// request packet, but will be set if we're building from a vec.
    connection_id: Option< u16 >,
}

impl RequestPacket {
    /// Constructs a new RequestPacket.
    pub fn new( resource_num: u16 ) -> Result<Self, packet::PacketError>
    {
        Ok( Self {
            resource_num: resource_num,
            connection_id: None,
        })
    }

    /// Convert the vector of bytes into a packet object
    pub fn from_vec(
        vec_packet: Vec<u8>
    ) -> Result<Self, packet::PacketError> {
        if 10 != vec_packet.len() {
            return Err(
                packet::PacketError::MalformedPacket(
                    "Packet too large".to_string()
                )
            );
        }

        let connection_id: u16 = ((vec_packet[5] as u16) << 8)
            | (vec_packet[6] as u16);
        let resource_num: u16 = ((vec_packet[8] as u16) << 8)
            | (vec_packet[9] as u16);

        Ok( Self {
            resource_num: resource_num,
            connection_id: Some( connection_id ),
        })
    }

    /// Convert the packet into a vector of bytes that can be sent over the 
    /// wire
    pub fn to_vec( &self ) -> Vec<u8>
    {
        let resource_num_big: u8 = ((self.resource_num >> 8) & 0xFF) as u8;
        let resource_num_small: u8 = (self.resource_num & 0xFF) as u8;

        let connection_id = match self.connection_id {
            Some(id) => id,
            None => {
                let mut rng = SmallRng::from_rng( &mut rand::rng() );
                rng.random()
            },
        };

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
    fn request_packet_serialize()
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

    #[test]
    fn request_packet_deserialize()
    {
        let mut pack = RequestPacket::new( 0xAB12 ).unwrap();
        pack.connection_id = Some( 0xEF98 );
        let pack_vec = pack.to_vec();

        let new_pack = RequestPacket::from_vec( pack_vec ).unwrap();
        assert_eq!( pack.connection_id, new_pack.connection_id );
        assert_eq!( pack.resource_num, pack.resource_num );
    }
}
