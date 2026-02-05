# ZON
### The Zero-Overhead Notation for High-Performance Systems.

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Speed](https://img.shields.io/badge/speed-blazing-fire)

## The Problem
Data serialization implies a cost. Formats like JSON force CPUs to burn cycles parsing text, allocating strings, and managing garbage, turning every data access into a computation. For high-frequency systems, this latency is unacceptable.

**ZON** changes the paradigm. It maps files directly to memory. By using pointer-less relative offsets and strict 64-byte alignment, the on-disk format *is* the in-memory representation. It is cache-line friendly, mmap-ready, and requires zero parsing.

## The Evidence
Benchmarks comparing ZON against standard JSON deserialization for a composite game entity (`Player` struct):

| Format | Mean Access Time | Throughput | Speedup |
|:-------|:-----------------|:-----------|:--------|
| **JSON** | ~117.43 ns | ~8.5 M ops/s | 1x |
| **ZON**  | **~18.83 ns** | **~53.1 M ops/s** | **6.2x** |

*> Benchmark conducted on a strictly aligned composite workload.*

## Quick Start

### Usage

```rust
use zon_format::{ZonWriter, ZonReader};

fn main() {
    // 1. Write Data
    let mut writer = ZonWriter::new();
    
    // Write a string (returns offset to length prefix)
    let name_offset = writer.write_string("Hero");
    
    // Write an integer
    let score_offset = writer.write_u32(9001);
    
    // Link the root of the file to the name
    writer.set_root(name_offset);
    
    // 2. Read Data (Zero-Copy)
    let buffer = writer.as_bytes();
    let reader = ZonReader::new(buffer).expect("Valid ZON file");
    
    // Read the root pointer (stored at offset 8 in header)
    let root_ptr = reader.read_u32(8).unwrap();
    
    // Resolve the string directly from the buffer without allocation
    let name = reader.read_string(root_ptr).unwrap();
    
    assert_eq!(name, "Hero");
}
```

## Philosophy
**Data should not be parsed. It should be read.**

ZON is built for systems where latency is the primary constraint:
*   High-Frequency Trading (HFT)
*   Game Engines (Entity Component Systems)
*   Large Scale Distributed Systems

By eliminating the transformation layer between disk and memory, we free the CPU to do actual work.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
zon-format = { path = "." }
```
