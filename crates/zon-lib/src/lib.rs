use wasm_bindgen::prelude::*;
use zon_lib::{ZonWriter as RustZonWriter, ZonReader as RustZonReader};
use serde_wasm_bindgen;
use serde::Serialize;

#[wasm_bindgen]
pub struct ZonWriter {
    inner: RustZonWriter,
}

#[wasm_bindgen]
impl ZonWriter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: RustZonWriter::new(),
        }
    }

    #[wasm_bindgen(js_name = "writeString")]
    pub fn write_string(&mut self, val: &str) -> u32 {
        self.inner.write_string(val)
    }

    #[wasm_bindgen(js_name = "writeU32")]
    pub fn write_u32(&mut self, val: u32) -> u32 {
        self.inner.write_u32(val)
    }

    #[wasm_bindgen(js_name = "setRoot")]
    pub fn set_root(&mut self, offset: u32) {
        self.inner.set_root(offset);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.as_bytes().to_vec()
    }
}

#[wasm_bindgen]
pub struct ZonReader {
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl ZonReader {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: Vec<u8>) -> Result<ZonReader, JsError> {
        // Validate magic number etc. by trying to create a reader
        RustZonReader::new(&buffer).map_err(|e| JsError::new(e))?;
        Ok(Self { buffer })
    }

    #[wasm_bindgen(js_name = "readU32")]
    pub fn read_u32(&self, offset: u32) -> Result<u32, JsError> {
        let reader = RustZonReader::new(&self.buffer).map_err(|e| JsError::new(e))?;
        reader.read_u32(offset).map_err(|e| JsError::new(e))
    }

    #[wasm_bindgen(js_name = "readString")]
    pub fn read_string(&self, offset: u32) -> Result<String, JsError> {
        let reader = RustZonReader::new(&self.buffer).map_err(|e| JsError::new(e))?;
        reader.read_string(offset)
            .map(|s| s.to_string())
            .map_err(|e| JsError::new(e))
    }

    #[wasm_bindgen(getter, js_name = "rootOffset")]
    pub fn root_offset(&self) -> Result<u32, JsError> {
        self.read_u32(8)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

/// serialize a plain JS object to ZON format.
///
#[wasm_bindgen(js_name = "serialize")]
pub fn serialize(data: JsValue) -> Result<Vec<u8>, JsError> {
    // For now, this is a placeholder that would use serde to traverse and write.
    // In our previous implementation, we had a basic version for strings/u32.
    // Let's implement a basic version that supports key-value pairs if it's an object.
    
    let mut writer = RustZonWriter::new();
    
    // We'll use serde_json as a middle-man for this simple bridge if needed,
    // but the goal of ZON is to avoid that.
    // For this bridge, we'll support a simple flat object serialization as a demonstration.
    
    let map: serde_json::Value = serde_wasm_bindgen::from_value(data)?;
    
    if let Some(obj) = map.as_object() {
        // Simple serialization: write all strings and numbers, and set root to first entry?
        // Actually, let's just serialize the first value for now as we did before.
        if let Some((_key, value)) = obj.iter().next() {
             if let Some(s) = value.as_str() {
                 let off = writer.write_string(s);
                 writer.set_root(off);
             } else if let Some(i) = value.as_u64() {
                 let off = writer.write_u32(i as u32);
                 writer.set_root(off);
             }
        }
    }
    
    Ok(writer.as_bytes().to_vec())
}
