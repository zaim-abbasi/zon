

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

pub struct ZonReader<'a> {
    buffer: &'a [u8],
}

impl<'a> ZonReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Result<Self, &'static str> {
        if buffer.len() < std::mem::size_of::<ZonHeader>() {
            return Err("Buffer too small for ZonHeader");
        }

        // Check magic number (offset 0..4)
        // Manual check to avoid casting requirement, though casting header is usually fine for read
        // But let's stick to safe slice checks as requested by "Verification"
        let magic_slice = &buffer[0..4];
        let magic = u32::from_le_bytes(magic_slice.try_into().unwrap());
        if magic != ZON_MAGIC {
            return Err("Invalid Magic Number");
        }

        Ok(Self { buffer })
    }

    pub fn read_u32(&self, offset: u32) -> Result<u32, &'static str> {
        let start = offset as usize;
        let end = start + 4;
        if end > self.buffer.len() {
            return Err("Read out of bounds");
        }
        let slice = &self.buffer[start..end];
        Ok(u32::from_le_bytes(slice.try_into().unwrap()))
    }

    pub fn read_string(&self, offset: u32) -> Result<&'a str, &'static str> {
        // Read length
        let len_offset = offset;
        let len = self.read_u32(len_offset)?;
        
        let start = (offset + 4) as usize;
        let end = start + len as usize;
        
        if end > self.buffer.len() {
             return Err("String read out of bounds");
        }
        
        let str_bytes = &self.buffer[start..end];
        std::str::from_utf8(str_bytes).map_err(|_| "Invalid UTF-8")
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

    #[test]
    fn test_end_to_end() {
        let mut writer = ZonWriter::new();
        
        let val: u32 = 123456;
        let text = "Zaim";
        
        // Write data
        let val_off = writer.write_u32(val);
        let text_off = writer.write_string(text);
        
        // Set root to the string
        writer.set_root(text_off);
        
        // Finalize buffer
        let buffer = writer.as_bytes();
        
        // Read back
        let reader = ZonReader::new(buffer).expect("Valid buffer");
        
        // Read u32
        let read_val = reader.read_u32(val_off).expect("Read u32");
        assert_eq!(read_val, val);
        
        // Read string
        let read_text = reader.read_string(text_off).expect("Read string");
        assert_eq!(read_text, text);
        
        // Read root implicitly (via offset 8, though we didn't expose get_root in reader yet, we can check it via read_u32(8))
        let root_ptr = reader.read_u32(8).expect("Read root");
        assert_eq!(root_ptr, text_off);
        
        // Check reading string from root pointer
        let read_root_text = reader.read_string(root_ptr).expect("Read string from root");
        assert_eq!(read_root_text, text);
    }
}
