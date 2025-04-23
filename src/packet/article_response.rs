use crate::packet;


pub struct ArticleResponsePacket
{
    /// Contents of article
    article: String,
    /// Connection ID
    connection_id: u16,
}

impl ArticleResponsePacket {
    /// Constructs a new ArticleResponsePacket.
    ///
    /// `connection_id` is typically the ID sent by the RequestPacket.
    pub fn new(
        article: String,
        connection_id: u16,
    ) -> Result<Self, packet::PacketError>
    {
        Ok( Self {
            article: article,
            connection_id: connection_id,
        })
    }

    /// Convert the vector of bytes into a packet object
    pub fn from_vec(
        vec_packet: Vec<u8>
    ) -> Result<Self, packet::PacketError> {
        let connection_id: u16 = ((vec_packet[5] as u16) << 8)
            | (vec_packet[6] as u16);
        let article_bytes = &vec_packet[ 8 .. vec_packet.len() ];
        let article = String::from_utf8( article_bytes.to_vec() ).unwrap();

        Ok( Self {
            article: article,
            connection_id: connection_id,
        })
    }

    /// Convert the packet into a vector of bytes that can be sent over the 
    /// wire
    pub fn to_vec( &self ) -> Vec<u8>
    {
        let mut payload: Vec<u8> = vec![];
        payload.extend( self.article.as_bytes() );
        packet::packet_to_vec(
            packet::PacketType::ArticleResponse,
            payload,
            self.connection_id,
        ).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_response_packet()
    {
        let pack_result = ArticleResponsePacket::new( "foobarbaz".to_string(),
            0x1234 );

        assert_eq!( pack_result.is_ok(), true );
    }

    #[test]
    fn article_response_packet_serialize()
    {
        let pack = ArticleResponsePacket::new( "foobarbaz".to_string(),
            0x1234 ).unwrap();
        let pack_vec = pack.to_vec();

        let mut expect_vec: Vec<u8> = vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            packet::PacketType::ArticleResponse.value(),
            // Connection ID
            0x12,
            0x34,
            9, // Length
        ];
        expect_vec.extend( "foobarbaz".as_bytes() );

        assert_eq!( pack_vec, expect_vec );
    }

    #[test]
    fn article_response_packet_deserialize()
    {
        let pack = ArticleResponsePacket::new(
            "foo".to_string(),
            0xAB12,
        ).unwrap();
        let pack_vec = pack.to_vec();

        let new_pack = ArticleResponsePacket::from_vec( pack_vec ).unwrap();
        assert_eq!( pack.connection_id, new_pack.connection_id );
        assert_eq!( pack.article, new_pack.article );
    }
}
