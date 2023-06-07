pub use ahash::AHasher;
use byteorder::{ByteOrder, LittleEndian};
use murmur3::murmur3_x64_128;
use rand::Rng;
use seahash::SeaHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

pub fn hash64(mut bytes: &[u8]) -> u64 {
    let mut out = [0; 16];
    murmur3_x64_128(&mut bytes, 0, &mut out);
    // in most cases we run on little endian target
    LittleEndian::read_u64(&out[0..8])
}

#[derive(Debug, Default)]
struct MurmurHasher(u64);

impl Hasher for MurmurHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 ^= hash64(bytes);
    }
}

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = (0..length)
                .map(|_| rng.gen_range(0..36))
                .map(|n| if n < 26 { (n + 97) as u8 } else { (n - 26 + 48) as u8 } as char)
                .collect();
    chars.iter().collect()
}

const KEY_NUM: usize = 100000;
const KEY_LEN: usize = 100;
const BUCKET_LEN: usize = 128;

fn test_hash_speed<F>(hash_function: F, keys: &[String]) -> Duration
where
    F: Fn(&str) -> u64,
{
    let start_time = Instant::now();
    for key in keys {
        hash_function(&key);
    }

    start_time.elapsed()
}

fn test_hash_dist<F>(hash_function: F, keys: &[String]) -> f64
where
    F: Fn(&str) -> u64,
{
    let mut buckets = vec![0; BUCKET_LEN];
    for key in keys {
        let idx = hash_function(&key) as usize % BUCKET_LEN;
        buckets[idx] += 1;
    }

    let mean = buckets.iter().sum::<usize>() as f64 / BUCKET_LEN as f64;
    let variance = buckets
        .iter()
        .map(|n| {
            let diff = *n as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / BUCKET_LEN as f64;

    // std_dev
    variance.sqrt()
}

fn main() {
    let keys: Vec<_> = (0..KEY_NUM)
        .map(|_| generate_random_string(KEY_LEN))
        .collect();

    println!("key num is {}, len of each key:{}", KEY_NUM, KEY_LEN);

    let a_hash = |x: &str| -> u64 {
        let mut hasher = AHasher::default();
        x.hash(&mut hasher);
        hasher.finish()
    };
    let sea_hash = |x: &str| -> u64 {
        let mut hasher = SeaHasher::default();
        x.hash(&mut hasher);
        hasher.finish()
    };
    let sea_hash2 = |x: &str| -> u64 { seahash::hash(x.as_bytes()) };

    let defalut_hash = |x: &str| -> u64 {
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        hasher.finish()
    };
    let murmur_hash = |x: &str| -> u64 {
        let mut hasher = MurmurHasher::default();
        x.hash(&mut hasher);
        hasher.finish()
    };
    let murmur_hash2 = |x: &str| -> u64 { hash64(x.as_bytes()) };

    println!("----Build_cost(ns)------");
    println!(
        "default:{}, ahash:{}, murmur:{}, murmur2:{}, seahash:{}, seahash2:{}",
        test_hash_speed(defalut_hash, &keys).as_nanos(),
        test_hash_speed(a_hash, &keys).as_nanos(),
        test_hash_speed(murmur_hash, &keys).as_nanos(),
        test_hash_speed(murmur_hash2, &keys).as_nanos(),
        test_hash_speed(sea_hash, &keys).as_nanos(),
        test_hash_speed(sea_hash2, &keys).as_nanos(),
    );
    println!("----Std_dev------");
    println!(
        "default:{:.3}, ahash:{:.3}, murmur:{:.3}, murmur2:{:.3}, seahash:{:.3}, seahash2:{:.3}",
        test_hash_dist(defalut_hash, &keys),
        test_hash_dist(a_hash, &keys),
        test_hash_dist(murmur_hash, &keys),
        test_hash_dist(murmur_hash2, &keys),
        test_hash_dist(sea_hash, &keys),
        test_hash_dist(sea_hash2, &keys),
    );
}
