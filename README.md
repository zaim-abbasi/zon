<div align="center">
  
# 🎯 ZON

### Stop Parsing. Start Reading.

**A Zero-Copy, Schema-Less Binary Format**  
**6.2× Faster than JSON**

[![Crates.io](https://img.shields.io/crates/v/zon-lib.svg)](https://crates.io/crates/zon-lib)
[![NPM](https://img.shields.io/npm/v/@zon-lib/zon.svg)](https://www.npmjs.com/package/@zon-lib/zon)
[![Documentation](https://img.shields.io/badge/docs-zon.mintlify.app-10B981?style=flat&logo=mintlify&logoColor=white)](https://zon.mintlify.app)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Quick Start](#-quick-install) • [Why ZON?](#-the-philosophy) • [Benchmarks](#-benchmarks) • [Documentation](https://zon.mintlify.app)

</div>

---

## ⚡ ZON in 30 Seconds

> **ZON removes the "JSON tax."**  
> Your data becomes a high-speed binary format that browsers and servers can read **instantly**—without the CPU overhead of parsing.

```javascript
import { ZonReader } from '@zon-lib/zon';

async function loadData() {
  const res = await fetch('https://api.example.com/stats.zon');
  const buffer = new Uint8Array(await res.arrayBuffer());

  // ✨ wrap the buffer — no parsing happens here
  const stats = new ZonReader(buffer);

  // 🚀 read data at o(1) speed
  console.log(stats.read_u32(8)); 
}
```

**The Result?** 6.2× faster data access with zero parsing overhead.

---

## 🎯 Quick Install

```bash
# node.js / web (wasm)
npm install @zon-lib/zon

# rust (systems)
cargo add zon-lib

# cli inspector
cargo install zon-inspector
```

---

## 💡 The Philosophy

Modern applications suffer from a **"Parsing Tax."** Whether JSON or Protobuf, your CPU wastes massive cycles translating text into memory before you can even use it.

**ZON (Zero-Overhead Notation) eliminates this translation step.**

The binary format on disk is the **exact same layout** your CPU requires in memory.

**Core Principles:**
- 🎯 **Zero-Copy Access** — We don't parse. We map directly to memory.
- 📝 **Schema-Less** — No `.proto` files. No code generation.
- 🌍 **Universal** — Native speed in Rust. Instant bridging in WASM.

---

## 🌐 Why for Web?

**Lower Battery Drain** — Less CPU usage means better battery life for mobile users  
**Zero UI Lag** — Large datasets (maps, 3D models, telemetry) load without freezing  
**Edge Ready** — Minimal memory footprint for Vercel Functions and Cloudflare Workers

---

## 📊 Benchmarks

**ZON vs. JSON** *(Accessing a composite `Player` struct)*

<table>
<thead>
<tr>
<th>Format</th>
<th align="right">Mean Access Time</th>
<th align="right">Throughput</th>
<th align="center">Speedup</th>
</tr>
</thead>
<tbody>
<tr>
<td><b>JSON</b></td>
<td align="right">~117.43 ns</td>
<td align="right">~8.5 M ops/s</td>
<td align="center">1×</td>
</tr>
<tr>
<td><b>ZON</b></td>
<td align="right"><b>~18.83 ns</b></td>
<td align="right"><b>~53.1 M ops/s</b></td>
<td align="center"><b>🚀 6.2×</b></td>
</tr>
</tbody>
</table>

> *Benchmark conducted on a strictly aligned composite workload on a consumer workstation.*

**What this means for you:**
- **Web Apps:** Load 6× more data in the same time budget
- **APIs:** Serve 6× more requests with the same infrastructure
- **Mobile:** Save precious battery and reduce heat

---

## 🛠️ Usage

### 🌐 Web (Browser & Node.js)
*Instant data access with zero parsing lag.*

```javascript
import { serialize_to_zon, ZonReader } from '@zon-lib/zon';

// step 1: serialize to binary (server-side)
const binary = serialize_to_zon({ name: "Hero", hp: 100 });

// step 2: zero-copy read (client-side)
// 🔥 no parsing occurs here — we simply wrap the memory buffer
const reader = new ZonReader(binary);

// step 3: o(1) direct access
const root = reader.get_root();
console.log(reader.read_string(root)); // "hero"
```

**Key Benefits:**
- ✅ No `JSON.parse()` overhead
- ✅ Instant data availability
- ✅ Lower memory pressure

---

### ⚙️ Systems (Rust)
*The core engine for HFT, Game Engines, and System Tools.*

```rust
use zon_lib::{ZonWriter, ZonReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. write with cache-line alignment
    let mut writer = ZonWriter::new();
    let name_off = writer.write_string("Hero");
    
    // 2. map bytes directly
    let buffer = writer.as_bytes();
    let reader = ZonReader::new(buffer)?;
    
    // 3. access with zero allocations
    let name = reader.read_string(name_off)?;
    println!("Name: {}", name);
    
    Ok(())
}
```

**Performance Characteristics:**
- ✅ Zero allocations in hot path
- ✅ CPU cache-friendly (64-byte aligned)
- ✅ Safe concurrent access

---

### 🔍 CLI Inspector
*Visualize `.zon` files without writing code.*

```bash
# install once
cargo install zon-inspector

# inspect any .zon file
zon-inspector data.zon
```

**Output Example:**
```
📦 ZON File: data.zon
├─ Size: 1.2 KB
├─ Entries: 42
└─ Root Object
   ├─ name: "Hero"
   └─ hp: 100
```

---

## 📚 Documentation

| Platform | Link |
|:---------|:-----|
| 🚀 **Web Guide** — Quick Start, Performance Patterns, Integration Examples | [**zon.mintlify.app**](https://zon.mintlify.app) |
| ⚙️ **Rust API Docs** — Complete API Reference, Type Specifications, Memory Layout | [**docs.rs/zon-lib**](https://docs.rs/zon-lib) |

---

## 🌍 Real-World Use Cases

| Use Case | Description |
|:---------|:------------|
| 📈 **High-Frequency Trading** | Nanosecond order book updates without parsing overhead |
| 🎮 **Multiplayer Games** | Synchronize thousands of entities with zero lag |
| 📊 **Telemetry Ingest** | Massive logging without JSON stringification cost |

**Also Great For:**  
🗺️ Geographic data (maps, tiles, GeoJSON) • 🤖 ML inference pipelines • 📡 IoT sensor streams • 🎬 Video/audio metadata

---

## 🤝 Contributing

**Core Principles:**
- 🎯 **Zero-Copy** — No memory allocations in the hot path
- ⚡ **64-Byte Aligned** — Respect CPU cache line architecture  
- 🔒 **Memory Safe** — Leverage Rust's type system

**Development Setup:**

```bash
# test core rust
cargo test --workspace

# build webassembly package
cd crates/zon-lib && wasm-pack build --target nodejs
```

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Built with ❤️ by **Zaim Abbasi** • Released under the MIT License

</div>