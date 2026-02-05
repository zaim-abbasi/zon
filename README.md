# ZON
### The Zero-Overhead Notation for High-Performance Systems.

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Speed](https://img.shields.io/badge/speed-blazing-fire)

**6.2x Faster than JSON.**
ZON maps files directly to memory using pointer-less relative offsets and strict 64-byte alignment. No parsing. No allocation. No garbage collection.

```text
JSON (Traditional):
[Disk] -> [Read String] -> [Parse Text] -> [Allocate Object] -> [Memory] 🐢

ZON (Zero-Overhead):
[Disk] ------------------ (mmap) -------------------------> [Memory] 🚀

```

## The Evidence

Benchmarks comparing ZON against standard JSON deserialization for a composite game entity (`Player` struct):

| Format | Mean Access Time | Throughput | Speedup |
| --- | --- | --- | --- |
| **JSON** | ~117.43 ns | ~8.5 M ops/s | 1x |
| **ZON** | **~18.83 ns** | **~53.1 M ops/s** | **6.2x** |

*> Benchmark conducted on a strictly aligned composite workload.*

## How It Works (Binary Anatomy)

ZON is not a text format; it is a memory dump. Here is the byte-level layout of a file containing the string `"Hero"` as the root.

**The Concept:** To read "Hero", the CPU jumps to the `Root Ptr`, reads the length, and consumes the bytes. Zero search.

| Offset (Hex) | Bytes (Hex) | Meaning |
| --- | --- | --- |
| `0x00` | `5A 4F 4E 40` | **Header** (Magic: "ZON@" + Version) |
| `0x08` | `40 00 00 00` | **Root Pointer** (Offset 64 / 0x40) |
| `0x0C` | `00 00 ...` | **Padding** (Align to 64 bytes) |
| `0x40` | `04 00 00 00` | **String Length** (4 bytes) |
| `0x44` | `48 65 72 6F` | **Data** ("Hero") |

## Usage

```rust
use zon_format::{ZonWriter, ZonReader};

fn main() {
    // 1. Write Data (Imperative Builder)
    let mut writer = ZonWriter::new();
    let name_offset = writer.write_string("Hero");
    
    // Set the entry point of the file
    writer.set_root(name_offset);
    
    // 2. Read Data (Zero-Copy)
    // In production, this buffer comes from a memory-mapped file
    let buffer = writer.as_bytes(); 
    let reader = ZonReader::new(buffer).expect("Valid ZON file");
    
    // Instant Access: Jump straight to the data
    let root_ptr = reader.read_u32(8).unwrap();
    let name = reader.read_string(root_ptr).unwrap();
    
    assert_eq!(name, "Hero");
}

```

## Philosophy

**Data should not be parsed. It should be read.**

ZON is engineered for systems where latency is the primary constraint:

* **High-Frequency Trading (HFT):** Order books and market data.
* **Game Engines:** Entity Component Systems (ECS) and asset loading.
* **Distributed Systems:** High-throughput telemetry.

By eliminating the translation layer between disk and memory, we free the CPU to do actual work.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
zon-format = "0.1.0"

```
