
pub struct CRC32(pub u32);
pub struct SHA1(pub [u8; 20]);

pub struct Entry {
    pub crc: CRC32,
    pub sha1: SHA1,
    pub name: String
}
