use crate::header::{ZonHeader, ZON_MAGIC};
use std::str;

pub struct ZonReader<'a> {
    buffer: &'a [u8],
}

impl<'a> ZonReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Result<Self, &'static str> {
        if buffer.len() < std::mem::size_of::<ZonHeader>() {
            return Err("Buffer too small for ZonHeader");
        }

        // check magic number (offset 0..4)
        // manual check to avoid casting requirement, though casting header is usually fine for read
        // but let's stick to safe slice checks as requested by "Verification"
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
        // read length
        let len_offset = offset;
        let len = self.read_u32(len_offset)?;
        
        let start = (offset + 4) as usize;
        let end = start + len as usize;
        
        if end > self.buffer.len() {
             return Err("String read out of bounds");
        }
        
        let str_bytes = &self.buffer[start..end];
        str::from_utf8(str_bytes).map_err(|_| "Invalid UTF-8")
    }
}
