use zon_lib::{ZonWriter, ZonReader};

fn main() {
    println!("Running ZON Basic Example...");

    // 1. write
    let mut writer = ZonWriter::new();
    let text_off = writer.write_string("Hello Zero-Copy World!");
    let num_off = writer.write_u32(42);
    
    writer.set_root(text_off);
    
    println!("Written {} bytes.", writer.len());
    
    // 2. read
    let buffer = writer.as_bytes();
    let reader = ZonReader::new(buffer).expect("Buffer verification failed");
    
    let root = reader.read_u32(8).unwrap();
    let text = reader.read_string(root).unwrap();
    let num = reader.read_u32(num_off).unwrap();
    
    println!("Read Root String: {}", text);
    println!("Read Number: {}", num);
    
    assert_eq!(text, "Hello Zero-Copy World!");
    assert_eq!(num, 42);
    
    println!("Success!");
}
