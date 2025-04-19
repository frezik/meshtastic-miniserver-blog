use crate::packet;

pub struct DirectoryEntry
{
    entry_id: u16,
    name: String,
}

pub struct DirectoryResponsePacket
{
    entries: Vec<DirectoryEntry>,
}

impl DirectoryResponsePacket
{
    pub fn new(
        entries: Vec<DirectoryEntry>,
    ) -> Result<Self, packet::PacketError >
    {
        Ok( Self {
            entries: entries,
        })
    }

    pub fn to_vec( &self ) -> Vec<u8>
    {
        let mut payload: Vec<u8> =  vec![];
        for entry in &self.entries {
            payload.push( ((entry.entry_id >> 8) & 0xFF) as u8 );
            payload.push( (entry.entry_id & 0xFF) as u8 );
            payload.extend( entry.name.as_bytes() );
        }

        packet::packet_to_vec(
            packet::PacketType::DirectoryResponse,
            payload,
        ).unwrap()
    }
}


#[cfg(test)]
mod tets {
    use super::*;

    #[test]
    fn directory_response_packet()
    {
        let pack_result = DirectoryResponsePacket::new( vec![
            DirectoryEntry {
                entry_id: 0xAABB,
                name: "Foo".to_string(),
            }
        ]);

        assert_eq!( pack_result.is_ok(), true );
    }
}
