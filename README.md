<div align="center">
  <h1>ZON</h1>
  
  **A Zero-Copy, Schema-Less Binary Format. 6.2x Faster than JSON.**
  
  [![Crates.io](https://img.shields.io/crates/v/zon-lib.svg)](https://crates.io/crates/zon-lib)
  [![NPM](https://img.shields.io/npm/v/@zaim-abbasi/zon-wasm.svg)](https://www.npmjs.com/package/@zaim-abbasi/zon-wasm)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
</div>

---

## ⚡ Quick Install

| **Core Library (Rust)** | **CLI Inspector Tool** | **Node.js Bridge** |
|:---:|:---:|:---:|
| `cargo add zon-lib` | `cargo install zon-inspector` | `npm install @zaim-abbasi/zon-wasm` |

---

## 🚀 Why ZON?

Data should not be parsed. **It should be read.**

Modern applications waste massive amounts of CPU cycles parsing text (JSON) or decoding schemas (Protobuf). **ZON** maps files directly to memory. By using pointer-less relative offsets and strict 64-byte alignment, the on-disk format **is** the in-memory representation.

| Feature | JSON | Protobuf | **ZON** |
|:---|:---:|:---:|:---:|
| **Zero-Copy Access** | ❌ | ❌ | ✅ |
| **Schema-Less** | ✅ | ❌ | ✅ |
| **Parsing Overhead** | High | Medium | **None** |
| **WASM Ready** | ✅ | ✅ | **Native** |

---

## 📊 Performance

Benchmarks comparing ZON against standard JSON deserialization for a composite game entity (`Player` struct) on a consumer workstation.

| Format | Mean Access Time | Throughput | Speedup |
|:-------|:-----------------|:-----------|:--------|
| **JSON** | ~117.43 ns | ~8.5 M ops/s | 1x |
| **ZON** | **~18.83 ns** | **~53.1 M ops/s** | **6.2x** |

> *Benchmark conducted on a strictly aligned composite workload.*

---

## 🛠️ Usage Guide

### 1. For Node.js / Web Backends (WASM)
*Recommended for high-performance web servers, distributed ingestors, and rapid prototyping.*

```javascript
const { serialize_to_zon, ZonReaderWasm } = require('@zaim-abbasi/zon-wasm');

// 1. serialize object to binary
const data = { name: "User1", score: 100 };
const buffer = serialize_to_zon(data);

// 2. read back with zero-copy efficiency
const reader = new ZonReaderWasm(buffer);
const root = reader.get_root();

console.log(reader.read_string(root)); // "User1"
```

### 2. For Rust Systems (Core Engine)
*Recommended for high-frequency trading, game engines, and system tools.*

```rust
use zon_lib::{ZonWriter, ZonReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Write data using the sdk
    let mut writer = ZonWriter::new();
    let name_off = writer.write_string("Hero");
    let score_off = writer.write_u32(9001);
    
    writer.set_root(name_off);
    
    // 2. Read data (zero-copy)
    let buffer = writer.as_bytes();
    let reader = ZonReader::new(buffer)?;
    
    // no parsing occurs here. access is direct pointer arithmetic.
    let root_offset = reader.read_u32(8)?; 
    Ok(())
}
```

### 3. The Inspector (CLI Tool)
*Use this to debug and visualize .zon files without writing code.*

```bash
# inspect a file to visualize its internal structure
zon-inspector data.zon
```

#### Example Output:

```text
[HEADER] Magic: ZON1 | Size: 48 bytes
[0x00] String: "Hero"
[0x10] Root Offset -> pointing to 0x00
```

---

## 📖 Documentation & API Reference

ZON is built for professional integration. For detailed API documentation, error types, and advanced memory layout specifications, please visit:

👉 [ZON Documentation on docs.rs](https://docs.rs/zon-lib)

---

## 🌍 Real-World Domains

ZON is designed as the core data engine for performance-critical industries, including:

**High-Frequency Trading (HFT)**: Where every microsecond counts for order book updates.

**Multiplayer Game Servers**: Synchronizing massive entity states with zero serialization overhead.

**Real-Time Analytics**: Ingesting high-volume telemetry without the CPU cost of JSON parsing.

---

<div align="center">
  <sub>Built with ❤️ by Zaim Abbasi. Released under the MIT License.</sub>
</div>
