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
    fn article_response_packet_deserialize()
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
}
