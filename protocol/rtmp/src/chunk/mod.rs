pub mod define;
pub mod errors;
pub mod packetizer;
pub mod unpacketizer;

// pub use chunk::{ChunkBasicHeader, ChunkHeader, ChunkInfo, ChunkMessageHeader};

use bytes::BytesMut;

//5.3.1.1
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChunkBasicHeader {
    pub format: u8,
    pub chunk_stream_id: u32,
}

impl ChunkBasicHeader {
    pub fn new(fmt: u8, csid: u32) -> ChunkBasicHeader {
        ChunkBasicHeader {
            format: fmt,
            chunk_stream_id: csid,
        }
    }
}

//5.3.1.2
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChunkMessageHeader {
    //save the absolute timestamp of chunk type 0.
    //or save the computed absolute timestamp of chunk type 1,2,3.
    pub timestamp: u32,
    pub msg_length: u32,
    pub msg_type_id: u8,
    pub msg_streamd_id: u32,
    // Save the timestamp delta of chunk type 1,2.
    // For chunk type 3, this field saves the timestamp
    // delta inherited from the previous chunk type 1 or 2.
    // NOTE: this value should be reset to 0 when the current chunk type is 0.
    pub timestamp_delta: u32,
    // This field will be set for type 0,1,2 .If the timestamp/timestamp delta >= 0xFFFFFF
    // then set this value to true else set it to false.
    // Note that when the chunk format is 3, this value will be inherited from
    // the most recent chunk 0, 1, or 2 chunk.(5.3.1.3 Extended Timestamp).
    pub is_extended_timestamp: bool,
}

impl ChunkMessageHeader {
    pub fn new(timestamp: u32, msg_length: u32, msg_type_id: u8, msg_stream_id: u32) -> Self {
        Self {
            timestamp,
            msg_length,
            msg_type_id,
            msg_streamd_id: msg_stream_id,
            timestamp_delta: 0,
            is_extended_timestamp: false,
        }
    }
}

pub struct ChunkHeader {
    pub basic_header: ChunkBasicHeader,
    pub message_header: ChunkMessageHeader,
}

impl Default for ChunkHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkHeader {
    pub fn new() -> ChunkHeader {
        ChunkHeader {
            basic_header: ChunkBasicHeader::new(0, 0),
            message_header: ChunkMessageHeader::new(0, 0, 0, 0),
        }
    }
}

// pub struct Chunk {
//     basic_header: ChunkBasicHeader,
//     message_header: ChunkMessageHeader,
//     raw_data: BytesMut,
// }

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChunkInfo {
    pub basic_header: ChunkBasicHeader,
    pub message_header: ChunkMessageHeader,
    pub payload: BytesMut,
}

impl Default for ChunkInfo {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, BytesMut::new())
    }
}

impl ChunkInfo {
    pub fn new(
        csid: u32,
        format: u8,
        timestamp: u32,
        msg_length: u32,
        msg_type_id: u8,
        msg_stream_id: u32,
        payload: BytesMut,
    ) -> Self {
        Self {
            basic_header: ChunkBasicHeader::new(format, csid),
            message_header: ChunkMessageHeader::new(
                timestamp,
                msg_length,
                msg_type_id,
                msg_stream_id,
            ),
            payload,
        }
    }
}

// impl Chunk {
//     pub fn chunk_read(&mut self, bytes: &[u8]) -> Result {
//         self.buffer.extend_from_slice(bytes);
//     }

//     pub fn read_basic_header(&mut self, bytes: &[u8]) -> Result<UnpackResult, ChunkUnpackError> {
//         if self.buffer.len() < 1 {
//             return Ok(UnpackResult::NotEnoughBytes);
//         }
//     }
// }
