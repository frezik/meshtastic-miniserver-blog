use crate::packet::article_response::ArticleResponsePacket;
use crate::packet::directory_response::DirectoryResponsePacket;
use crate::packet::error_response::ErrorResponsePacket;
use crate::packet::request::RequestPacket;

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

const PACKET_TYPE_REQUEST: u8 = 0x00;
const PACKET_TYPE_DIRECTORY_RESPONSE: u8 = 0x01;
const PACKET_TYPE_ARTICLE_RESPONSE: u8 = 0x02;
const PACKET_TYPE_ERROR_RESPONSE: u8 = 0x03;

/// Defines the different types of packets
pub enum PacketType {
    Request,
    DirectoryResponse,
    ArticleResponse,
    ErrorResponse,
}

impl PacketType {
    /// Transforms a packet type into its ID number
    fn value( &self ) -> u8
    {
        match *self {
            PacketType::Request => PACKET_TYPE_REQUEST,
            PacketType::DirectoryResponse => PACKET_TYPE_DIRECTORY_RESPONSE,
            PacketType::ArticleResponse => PACKET_TYPE_ARTICLE_RESPONSE,
            PacketType::ErrorResponse => PACKET_TYPE_ERROR_RESPONSE,
        }
    }
}

#[derive(Debug)]
/// Different types of errors
pub enum PacketError {
    MalformedPacket( String ),
    ProtocolVersion( u16 ),
    UnknownPacketType( u8 ),
    /// Payload too big. First number is the limit, second number is how big
    /// the payload is.
    PayloadTooLarge( u8, u32 ),
}

/// Different types of packeets
pub enum PacketResult {
    Request( RequestPacket ),
    DirectoryResponse( DirectoryResponsePacket ),
    ArticleResponse( ArticleResponsePacket ),
    ErrorResponse( ErrorResponsePacket ),
}

/// Transforms a packet type, payload, and connection ID into a vector of 
/// bytes to be sent over the wire.
///
/// Usually, this isn't called directly. Instead, call `to_vec()` on the 
/// packet object.
pub fn packet_to_vec(
    packet_type: PacketType,
    payload: Vec<u8>,
    connection_id: u16,
) -> Result< Vec<u8>, PacketError >
{
    let payload_len = payload.len();
    if payload_len > 255 {
        return Err( PacketError::PayloadTooLarge( 255, payload_len as u32 ) );
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


/// Transforms a vector of bytes into a packet object
pub fn vec_to_packet(
    vec_packet: Vec<u8>,
) -> Result< PacketResult, PacketError >
{
    if 8 > vec_packet.len() {
        return Err( PacketError::MalformedPacket(
            "Packet too short".to_string() ) );
    }

    let magic_num = ((vec_packet[0] as u16) << 8) | (vec_packet[1] as u16);
    if MAGIC_NUM != magic_num {
        return Err( PacketError::MalformedPacket(
            "Unrecognized magic number".to_string() ) );
    }

    let protocol_version = ((vec_packet[2] as u16) << 8)
        | (vec_packet[3] as u16);
    if PROTOCOL_VERSION < protocol_version {
        return Err( PacketError::ProtocolVersion( PROTOCOL_VERSION ) );
    }

    let packet_type = vec_packet[4];
    match packet_type {
        PACKET_TYPE_REQUEST => Ok(
            PacketResult::Request(
                RequestPacket::from_vec( vec_packet ).unwrap()
            )
        ),
        // TODO all of these
        //PACKET_TYPE_DIRECTORY_RESPONSE => Ok(
        //    DirectoryResponsePacket::from_vec( vec_packet )
        //),
        //PACKET_TYPE_ARTICLE_RESPONSE => Ok(
        //    ArticleResponsePacket::from_vec( vec_packet )
        //),
        //PACKET_TYPE_ERROR_RESPONSE => Ok(
        //    ErrorResponsePacket::from_vec( vec_packet )
        //),
        _ => Err(
            PacketError::UnknownPacketType( packet_type ) 
        ),
    }
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
