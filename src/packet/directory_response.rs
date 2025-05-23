use crate::packet;

const RECORD_SEPARATOR: u8 = 0x1E;

pub struct DirectoryEntry
{
    /// A unique ID for this entry in the directory
    entry_id: u16,
    /// A short string to describe the entry for meatsacks
    name: String,
}

pub struct DirectoryResponsePacket
{
    /// A list of directory entries
    entries: Vec<DirectoryEntry>,
    /// Connection ID
    connection_id: u16,
}

impl DirectoryResponsePacket
{
    /// Constructs a new DirectoryResponsePacket.
    ///
    /// `connection_id` is typically the ID sent by the RequestPacket.
    pub fn new(
        entries: Vec<DirectoryEntry>,
        connection_id: u16,
    ) -> Result<Self, packet::PacketError >
    {
        Ok( Self {
            entries: entries,
            connection_id: connection_id,
        })
    }

    /// Convert the vector of bytes into a packet object
    pub fn from_vec(
        vec_packet: Vec<u8>
    ) -> Result<Self, packet::PacketError> {
        let connection_id: u16 = ((vec_packet[5] as u16) << 8)
            | (vec_packet[6] as u16);

        let entry_strings = vec_packet.split( |i| RECORD_SEPARATOR == *i );
        let entries = entry_strings.map( |entry| {
            let entry_id = ((entry[0] as u16) << 8)
                | (entry[1] as u16);

            let entry_str_slice = &entry[ 2 .. entry.len() - 1 ];
            let mut entry_vec = vec![];
            entry_vec.extend_from_slice( entry_str_slice );
            let entry_str = String::from_utf8( entry_vec ).unwrap();

            DirectoryEntry {
                entry_id: entry_id,
                name: entry_str,
            }
        }).collect();

        Ok( Self {
            entries: entries,
            connection_id: connection_id,
        })
    }

    /// Convert the packet into a vector of bytes that can be sent over the 
    /// wire
    pub fn to_vec( &self ) -> Vec<u8>
    {
        let mut entry_payload: Vec<u8> = vec![];

        for entry in &self.entries {
            entry_payload.push( ((entry.entry_id >> 8) & 0xFF) as u8 );
            entry_payload.push( (entry.entry_id & 0xFF) as u8 );
            entry_payload.extend( entry.name.as_bytes() );
            entry_payload.push( RECORD_SEPARATOR );
        }

        packet::packet_to_vec(
            packet::PacketType::DirectoryResponse,
            entry_payload,
            self.connection_id,
        ).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_response_packet()
    {
        let pack_result = DirectoryResponsePacket::new(
            vec![
                DirectoryEntry {
                    entry_id: 0xAABB,
                    name: "Foo".to_string(),
                },
            ],
            0xCCDD,
        );

        assert_eq!( pack_result.is_ok(), true );
    }

    #[test]
    fn directory_single_entry()
    {
        let pack = DirectoryResponsePacket::new(
            vec![
                DirectoryEntry {
                    entry_id: 0xAABB,
                    name: "Foo".to_string(),
                },
            ],
            0xCCDD,
        ).unwrap();
        let pack_vec = pack.to_vec();

        let expect_vec: Vec<u8> = vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            packet::PacketType::DirectoryResponse.value(),
            0xCC, // connection ID BE
            0xDD, // connection ID LE
            6, // length
            0xAA, // entry ID BE
            0xBB, // entry ID LE
            0x46, // "F"
            0x6F, // "o"
            0x6F, // "o"
            0x1E, // record separator
        ];

        assert_eq!( pack_vec, expect_vec );
    }

    #[test]
    fn directory_multiple_entry()
    {
        let pack = DirectoryResponsePacket::new(
            vec![
                DirectoryEntry {
                    entry_id: 0xAABB,
                    name: "Foo".to_string(),
                },
                DirectoryEntry {
                    entry_id: 0xAABC,
                    name: "Bar".to_string(),
                },
            ],
            0xCCDD,
        ).unwrap();
        let pack_vec = pack.to_vec();

        let expect_vec: Vec<u8> = vec![
            packet::MAGIC_NUM_BE,
            packet::MAGIC_NUM_LE,
            packet::PROTOCOL_VERSION_BE,
            packet::PROTOCOL_VERSION_LE,
            packet::PacketType::DirectoryResponse.value(),
            0xCC, // connection ID BE
            0xDD, // connection ID LE
            12, // length
               
            0xAA, // entry ID BE
            0xBB, // entry ID LE
            0x46, // "F"
            0x6F, // "o"
            0x6F, // "o"
            0x1E, // record separator
                  
            0xAA, // entry ID BE
            0xBC, // entry ID LE
            0x42, // "B"
            0x61, // "a"
            0x72, // "r"
            0x1E, // record separator
        ];

        assert_eq!( pack_vec, expect_vec );
    }
}
