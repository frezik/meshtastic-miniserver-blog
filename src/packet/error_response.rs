use crate::packet;


pub struct ErrorResponsePacket
{
    /// Error ID
    error_id: u8,
    /// Connection ID
    connection_id: u16,
    /// Brief string describing the error
    err_str: String,
}

impl ErrorResponsePacket {
    /// Constructs a new DirectoryResponsePacket.
    ///
    /// `connection_id` is typically the ID sent by the RequestPacket.
    pub fn new(
        error_id: u8,
        connection_id: u16,
        err_str: String,
    ) -> Result<Self, packet::PacketError>
    {
        Ok( Self {
            error_id: error_id,
            connection_id: connection_id,
            err_str: err_str,
        })
    }

    /// Convert the vector of bytes into a packet object
    pub fn from_vec(
        vec_packet: Vec<u8>
    ) -> Result<Self, packet::PacketError> {
        let connection_id: u16 = ((vec_packet[5] as u16) << 8)
            | (vec_packet[6] as u16);
        let error_id: u8 = vec_packet[8];
        let error_bytes = &vec_packet[ 9 .. vec_packet.len() ];
        let err_str = String::from_utf8( error_bytes.to_vec() ).unwrap();

        Ok( Self {
            error_id: error_id,
            connection_id: connection_id,
            err_str: err_str,
        })
    }

    /// Convert the packet into a vector of bytes that can be sent over the 
    /// wire
    pub fn to_vec( &self ) -> Vec<u8>
    {
        let mut payload: Vec<u8> = vec![
            self.error_id,
        ];
        payload.extend( self.err_str.as_bytes() );
        packet::packet_to_vec(
            packet::PacketType::ErrorResponse,
            payload,
            self.connection_id,
        ).unwrap()
   }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_response_packet()
    {
        let pack_result = ErrorResponsePacket::new( 0xAB, 0x1234,
            "foobarbaz".to_string() );

        assert_eq!( pack_result.is_ok(), true );
    }

    #[test]
    fn error_response_packet_serialize()
    {
        let pack = ErrorResponsePacket::new( 0xAB, 0x1234,
            "foobarbaz".to_string(), ).unwrap();
        let pack_vec = pack.to_vec();

        let mut expect_vec: Vec<u8> = vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            packet::PacketType::ErrorResponse.value(),
            // Connection ID
            0x12,
            0x34,
            10, // Length
            0xAB, // Err ID
        ];
        expect_vec.extend( "foobarbaz".as_bytes() );

        assert_eq!( pack_vec, expect_vec );
    }

    #[test]
    fn error_response_packet_deserialize()
    {
        let pack = ErrorResponsePacket::new(
            0xBA,
            0xAB12,
            "foo".to_string(),
        ).unwrap();
        let pack_vec = pack.to_vec();

        let new_pack = ErrorResponsePacket::from_vec( pack_vec ).unwrap();
        assert_eq!( pack.connection_id, new_pack.connection_id );
        assert_eq!( pack.error_id, new_pack.error_id );
        assert_eq!( pack.err_str, new_pack.err_str );
    }
}
