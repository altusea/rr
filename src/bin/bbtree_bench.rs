// 单线程快速对比:std::BTreeMap vs bbtree::unsync::BBTreeMap
// 用 release 跑才有意义: cargo run --release --bin bbtree_bench
use bbtree::unsync::BBTreeMap;
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::{Duration, Instant};

const N: usize = 1_000_000;

fn keys() -> Vec<u64> {
    // splitmix64:固定种子,可重复
    let mut x = 0x2545_f491_4f6c_dd1d_u64;
    (0..N)
        .map(|_| {
            x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = x;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        })
        .collect()
}

fn ns_per_op(elapsed: Duration, ops: usize) -> f64 {
    elapsed.as_nanos() as f64 / ops as f64
}

fn main() {
    let keys = keys();
    let lookups = keys.clone(); // 全部命中的查找

    let t = Instant::now();
    let mut std_map = BTreeMap::new();
    for (i, k) in keys.iter().enumerate() {
        std_map.insert(*k, i as u64);
    }
    let std_insert = t.elapsed();

    let t = Instant::now();
    let mut fast = BBTreeMap::new();
    for (i, k) in keys.iter().enumerate() {
        fast.insert(*k, i as u64);
    }
    let fast_insert = t.elapsed();

    let t = Instant::now();
    let std_sum: u64 = lookups
        .iter()
        .map(|k| black_box(*black_box(&std_map).get(k).unwrap()))
        .sum();
    let std_get = t.elapsed();

    let t = Instant::now();
    let fast_sum: u64 = lookups
        .iter()
        .map(|k| black_box(*black_box(&fast).get(k).unwrap()))
        .sum();
    let fast_get = t.elapsed();

    assert_eq!(std_sum, fast_sum);
    assert_eq!(std_map.len(), fast.len());

    println!("{N} random u64 entries, single thread");
    println!(
        "insert: std {:7.1} ns/op | bbtree {:7.1} ns/op | {:4.2}x",
        ns_per_op(std_insert, N),
        ns_per_op(fast_insert, N),
        std_insert.as_secs_f64() / fast_insert.as_secs_f64()
    );
    println!(
        "get:    std {:7.1} ns/op | bbtree {:7.1} ns/op | {:4.2}x",
        ns_per_op(std_get, N),
        ns_per_op(fast_get, N),
        std_get.as_secs_f64() / fast_get.as_secs_f64()
    );
}
