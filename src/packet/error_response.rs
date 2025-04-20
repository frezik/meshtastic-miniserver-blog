use crate::packet;


pub struct ErrorResponsePacket
{
    error_id: u8,
    connection_id: u16,
    err_str: String,
}

impl ErrorResponsePacket {
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
    fn error_response_packet_deserialize()
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
}
