use crate::header::ZonHeader;

pub struct ZonWriter {
    pub(crate) buffer: Vec<u8>,
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

    /// Appends the 4 bytes of val to the buffer.
    /// Returns the offset (index) where those bytes were written.
    pub fn write_u32(&mut self, val: u32) -> u32 {
        let offset = self.buffer.len() as u32;
        self.buffer.extend_from_slice(&val.to_le_bytes());
        offset
    }

    /// First, append a 4-byte length (u32).
    /// Then, append the raw string bytes.
    /// Crucial: Append padding zeros until the buffer's total size is a multiple of 4 bytes.
    /// Returns the offset where the length was written.
    pub fn write_string(&mut self, val: &str) -> u32 {
        let start_offset = self.buffer.len() as u32;
        let len = val.len() as u32;
        
        // Write length
        self.buffer.extend_from_slice(&len.to_le_bytes());
        
        // Write string bytes
        self.buffer.extend_from_slice(val.as_bytes());
        
        // Add padding
        let current_len = self.buffer.len();
        let padding_needed = (4 - (current_len % 4)) % 4;
        for _ in 0..padding_needed {
            self.buffer.push(0);
        }
        
        start_offset
    }

    /// Updates the root offset in the header.
    /// The header is always at the start of the buffer.
    pub fn set_root(&mut self, offset: u32) {
        // root is at offset 8 (magic=0..4, version=4..8, root=8..12)
        if self.buffer.len() >= 12 {
            let bytes = offset.to_le_bytes();
            self.buffer[8] = bytes[0];
            self.buffer[9] = bytes[1];
            self.buffer[10] = bytes[2];
            self.buffer[11] = bytes[3];
        }
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

    #[test]
    fn test_write_primitive_and_string() {
        let mut writer = ZonWriter::new();
        
        // Write a u32
        let u32_val = 0x12345678;
        let u32_offset = writer.write_u32(u32_val);
        
        // Expect header is 64 bytes
        assert_eq!(u32_offset, 64);
        
        // Write a string "hello" (5 bytes)
        // Length 5 (4 bytes) + "hello" (5 bytes) = 9 bytes.
        // Padding needed: (4 - (9 % 4)) % 4 = (4 - 1) = 3 bytes padding.
        // Total added: 4 + 5 + 3 = 12 bytes.
        let string_val = "hello";
        let str_offset = writer.write_string(string_val);
        
        assert_eq!(str_offset, 64 + 4); // 68
        
        // Check buffer content
        {
            let bytes = writer.as_bytes();
            
            // Check u32 at 64
            let u32_slice = &bytes[64..68];
            assert_eq!(u32::from_le_bytes(u32_slice.try_into().unwrap()), u32_val);
            
            // Check string length at 68
            let len_slice = &bytes[68..72];
            assert_eq!(u32::from_le_bytes(len_slice.try_into().unwrap()), 5);
            
            // Check string bytes at 72
            let str_slice = &bytes[72..77];
            assert_eq!(str_slice, b"hello");
            
            // Check padding at 77..80
            assert_eq!(bytes[77], 0);
            assert_eq!(bytes[78], 0);
            assert_eq!(bytes[79], 0);
            
            // Check alignment
            assert_eq!(bytes.len() % 4, 0);
        }
        
        // Set root and verify
        writer.set_root(str_offset);
        
        {
            let bytes = writer.as_bytes();
            // Header root is at offset 8
            let root_slice = &bytes[8..12];
            assert_eq!(u32::from_le_bytes(root_slice.try_into().unwrap()), str_offset);
        }
    }
}
