use byteorder::ByteOrder;

use byteorder::LittleEndian;
use serde::{Deserialize, Serialize};

/// Represents a `stat` response
#[derive(Debug, Deserialize, Serialize)]
pub struct AdbStatResponse {
    /// File permissions
    pub file_perm: u32,
    /// File size, in bytes
    pub file_size: u32,
    /// File modification time
    pub mod_time: u32,
}

impl From<[u8; 12]> for AdbStatResponse {
    fn from(value: [u8; 12]) -> Self {
        Self {
            file_perm: LittleEndian::read_u32(&value[0..4]),
            file_size: LittleEndian::read_u32(&value[4..8]),
            mod_time: LittleEndian::read_u32(&value[8..]),
        }
    }
}
