use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use zon_lib::{ZonReader, ZonWriter};

#[derive(Serialize, Deserialize)]
struct Player {
    id: u32,
    score: u32,
    name: String,
}

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    // --- JSON Setup ---
    let player = Player {
        id: 123456,
        score: 999000,
        name: "Zaim".to_string(),
    };
    let json_str = serde_json::to_string(&player).unwrap();

    // --- ZON Setup ---
    let mut writer = ZonWriter::new();
    
    // 1. write Name
    let name_offset = writer.write_string("Zaim");
    
    // 2. pad to 64-byte alignment
    // we are at some offset. Note: write_string guarantees 4-byte aligned end.
    // we need to write padding zeros until writer.len() % 64 == 0.
    // effective padding can be done by writing u32(0) repeatedly since we are 4-byte aligned.
    while writer.len() % 64 != 0 {
        writer.write_u32(0);
    }
    
    let struct_start = writer.len() as u32;
    // 3. write fields
    writer.write_u32(player.id);
    writer.write_u32(player.score);
    writer.write_u32(name_offset);
    
    // 4. set Root
    writer.set_root(struct_start);
    
    let zon_buffer = writer.as_bytes();

    // --- Benchmarks ---

    group.bench_function("json_deserialize", |b| {
        b.iter(|| {
            let p: Player = serde_json::from_str(black_box(&json_str)).unwrap();
            black_box(p.id);
            black_box(p.score);
            black_box(&p.name);
        })
    });

    group.bench_function("zon_access", |b| {
        b.iter(|| {
            // includes validation step (ZonReader::new) as requested
            let reader = ZonReader::new(black_box(zon_buffer)).expect("valid buffer");
            
            // access Root
            let root = reader.read_u32(8).unwrap();
            
            // read fields
            let id = reader.read_u32(root).unwrap();
            let score = reader.read_u32(root + 4).unwrap();
            let name_ptr = reader.read_u32(root + 8).unwrap();
            let name = reader.read_string(name_ptr).unwrap();
            
            black_box(id);
            black_box(score);
            black_box(name);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_serialization);
criterion_main!(benches);
