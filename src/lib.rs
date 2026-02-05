

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
    // Reserved space to pad to 64 bytes.
    // u32 + u32 = 8 bytes used.
    // 64 - 8 = 56 bytes padding.
    _reserved: [u8; 56],
}

impl Default for ZonHeader {
    fn default() -> Self {
        Self {
            magic: ZON_MAGIC,
            version: 1,
            _reserved: [0; 56],
        }
    }
}

pub struct ZonWriter {
    buffer: Vec<u8>,
}

impl ZonWriter {
    pub fn new() -> Self {
        // Start with a default header
        let mut writer = Self {
            buffer: Vec::with_capacity(4096),
        };
        
        // Write the header immediately
        let header = ZonHeader::default();
        
        // Safety: ZonHeader is POD and repr(C).
        let header_slice = unsafe {
            std::slice::from_raw_parts(
                &header as *const ZonHeader as *const u8,
                std::mem::size_of::<ZonHeader>(),
            )
        };
        
        writer.buffer.extend_from_slice(header_slice);
        writer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for ZonWriter {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_writer_initialization() {
        let writer = ZonWriter::new();
        assert_eq!(writer.len(), 64);
        
        let bytes = writer.as_bytes();
        // Check magic number at offset 0 (little endian)
        assert_eq!(bytes[0], 0x21);
        assert_eq!(bytes[1], 0x4E);
        assert_eq!(bytes[2], 0x4F);
        assert_eq!(bytes[3], 0x5A);
    }
}
