use std::env;
use std::fs;
use std::process;

use zon_lib::ZonReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: zon-inspector <file.zon>");
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    let buffer = match fs::read(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };
    
    let reader = match ZonReader::new(&buffer) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error parsing ZON file: {}", e);
            process::exit(1);
        }
    };
    
    let json = to_json(&reader, &buffer);
    println!("{}", json);
}

/// traverses the ZON structure and converts it to JSON.
/// 
/// since ZON is a low-level format without self-describing schema,
/// we inspect the header and dump raw data.
fn to_json(reader: &ZonReader, buffer: &[u8]) -> String {
    // read header fields
    let magic = reader.read_u32(0).unwrap_or(0);
    let version = reader.read_u32(4).unwrap_or(0);
    let root = reader.read_u32(8).unwrap_or(0);
    
    // attempt to read root as a string (common case)
    let root_value = if root > 0 && root < buffer.len() as u32 {
        match reader.read_string(root) {
            Ok(s) => serde_json::json!({ "type": "string", "value": s }),
            Err(_) => {
                // try reading as u32
                match reader.read_u32(root) {
                    Ok(v) => serde_json::json!({ "type": "u32", "value": v }),
                    Err(_) => serde_json::json!({ "type": "unknown", "offset": root }),
                }
            }
        }
    } else {
        serde_json::json!(null)
    };
    
    let output = serde_json::json!({
        "header": {
            "magic": format!("0x{:08X}", magic),
            "version": version,
            "root_offset": root,
        },
        "root": root_value,
        "buffer_size": buffer.len(),
    });
    
    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}
