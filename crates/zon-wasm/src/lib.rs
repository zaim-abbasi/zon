use wasm_bindgen::prelude::*;
use zon_lib::{ZonReader, ZonWriter};

/// webAssembly wrapper for ZonWriter.
#[wasm_bindgen]
pub struct ZonWriterWasm {
    inner: ZonWriter,
}

#[wasm_bindgen]
impl ZonWriterWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: ZonWriter::new(),
        }
    }

    /// write a string and return its offset.
    #[wasm_bindgen]
    pub fn write_string(&mut self, val: &str) -> u32 {
        self.inner.write_string(val)
    }

    /// write a u32 and return its offset.
    #[wasm_bindgen]
    pub fn write_u32(&mut self, val: u32) -> u32 {
        self.inner.write_u32(val)
    }

    /// set the root offset in the header.
    #[wasm_bindgen]
    pub fn set_root(&mut self, offset: u32) {
        self.inner.set_root(offset);
    }

    /// get the current buffer length.
    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// convert buffer to Uint8Array for JS consumption.
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.as_bytes().to_vec()
    }
}

impl Default for ZonWriterWasm {
    fn default() -> Self {
        Self::new()
    }
}

/// webAssembly wrapper for ZonReader.
#[wasm_bindgen]
pub struct ZonReaderWasm {
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl ZonReaderWasm {
    /// create a new reader from a Uint8Array.
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: Vec<u8>) -> Result<ZonReaderWasm, JsError> {
        // validate by attempting to create a reader
        ZonReader::new(&buffer).map_err(|e| JsError::new(e))?;
        Ok(Self { buffer })
    }

    /// read a u32 at the given offset.
    #[wasm_bindgen]
    pub fn read_u32(&self, offset: u32) -> Result<u32, JsError> {
        let reader = ZonReader::new(&self.buffer).map_err(|e| JsError::new(e))?;
        reader.read_u32(offset).map_err(|e| JsError::new(e))
    }

    /// read a string at the given offset.
    #[wasm_bindgen]
    pub fn read_string(&self, offset: u32) -> Result<String, JsError> {
        let reader = ZonReader::new(&self.buffer).map_err(|e| JsError::new(e))?;
        reader.read_string(offset).map(|s| s.to_string()).map_err(|e| JsError::new(e))
    }

    /// get the root offset from the header.
    #[wasm_bindgen]
    pub fn get_root(&self) -> Result<u32, JsError> {
        self.read_u32(8)
    }

    /// get buffer length.
    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

/// serialize a plain JS object to ZON format.
/// 
/// supports objects with string and number fields.
/// returns a Uint8Array containing the ZON binary data.
#[wasm_bindgen]
pub fn serialize_to_zon(data: JsValue) -> Result<Vec<u8>, JsError> {
    // deserialize JsValue to serde_json::Value
    let json_value: serde_json::Value = serde_wasm_bindgen::from_value(data)
        .map_err(|e| JsError::new(&e.to_string()))?;
    
    let mut writer = ZonWriter::new();
    let mut field_offsets: Vec<(String, u32)> = Vec::new();
    
    // first pass: write all values and collect offsets
    if let serde_json::Value::Object(map) = &json_value {
        for (key, value) in map {
            let offset = match value {
                serde_json::Value::String(s) => writer.write_string(s),
                serde_json::Value::Number(n) => {
                    if let Some(u) = n.as_u64() {
                        writer.write_u32(u as u32)
                    } else if let Some(i) = n.as_i64() {
                        writer.write_u32(i as u32)
                    } else {
                        continue; // skip floats for now
                    }
                }
                _ => continue, // skip nested objects/arrays for now
            };
            field_offsets.push((key.clone(), offset));
        }
    }
    
    // pad to 64-byte alignment for struct
    while writer.len() % 64 != 0 {
        writer.write_u32(0);
    }
    
    let struct_start = writer.len() as u32;
    
    // second pass: write field name offsets and value offsets
    for (key, value_offset) in &field_offsets {
        let key_offset = writer.write_string(key);
        writer.write_u32(key_offset);
        writer.write_u32(*value_offset);
    }
    
    // write field count
    writer.write_u32(field_offsets.len() as u32);
    
    writer.set_root(struct_start);
    
    Ok(writer.as_bytes().to_vec())
}
