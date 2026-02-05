/// The magic number ZON! (0x5A4F4E21)
pub const ZON_MAGIC: u32 = 0x5A4F4E21;

/// The file header for ZON files.
/// Must be 64-byte aligned to ensure the start of the data segment
/// is also cache-line aligned.
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ZonHeader {
    pub magic: u32,
    pub version: u32,
    pub root: u32,
    // Reserved space to pad to 64 bytes.
    // u32 + u32 + u32 = 12 bytes used.
    // 64 - 12 = 52 bytes padding.
    _reserved: [u8; 52],
}

impl Default for ZonHeader {
    fn default() -> Self {
        Self {
            magic: ZON_MAGIC,
            version: 1,
            root: 0,
            _reserved: [0; 52],
        }
    }
}

impl ZonHeader {
    pub fn set_root(&mut self, offset: u32) {
        self.root = offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn test_zon_header_layout() {
        assert_eq!(align_of::<ZonHeader>(), 64, "ZonHeader must be 64-byte aligned");
        assert_eq!(size_of::<ZonHeader>(), 64, "ZonHeader size must be exactly 64 bytes");
    }

    #[test]
    fn test_magic_number() {
        let header = ZonHeader::default();
        assert_eq!(header.magic, 0x5A4F4E21);
    }
}
