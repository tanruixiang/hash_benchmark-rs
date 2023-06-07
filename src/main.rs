

/* We compared the speed difference between murmur3 and ahash for a string of
    length 10, and the results show that ahash has a clear advantage.
    Average time to DefaultHash a string of length 10: 33.6364 nanoseconds
    Average time to ahash a string of length 10: 19.0412 nanoseconds
    Average time to murmur3 a string of length 10: 33.0394 nanoseconds
    Warning: Do not use this hash in non-memory scenarios,
    One of the reasons is as follows:
    https://github.com/tkaitchuck/aHash/blob/master/README.md#goals-and-non-goals
*/
use byteorder::{ByteOrder, LittleEndian};
use murmur3::murmur3_x64_128;
use std::collections::hash_map::DefaultHasher;
use std::{collections, time::Instant};
use std::hash::{self, Hash, Hasher};
pub fn hash64(mut bytes: &[u8]) -> u64 {
    let mut out = [0; 16];
    murmur3_x64_128(&mut bytes, 0, &mut out);
    // in most cases we run on little endian target
    LittleEndian::read_u64(&out[0..8])
}

pub use ahash::AHasher;
use seahash::SeaHasher;
use rand::Rng;

fn main() {
        fn generate_random_string(length: usize) -> String {
            let mut rng = rand::thread_rng();
            let chars: Vec<char> = (0..length)
                .map(|_| rng.gen_range(0..36))
                .map(|n| if n < 26 { (n + 97) as u8 } else { (n - 26 + 48) as u8 } as char)
                .collect();
            chars.iter().collect()
        }

        fn test_hash_function<F>(hash_function: F, num_tests: usize, string_length: usize)
        where
            F: Fn(&str) -> u64,
        {
            let mut total_time = 0;
            for _ in 0..num_tests {
                let string_to_hash = generate_random_string(string_length);
                let start_time = Instant::now();
                hash_function(&string_to_hash);
                let end_time = start_time.elapsed();
                total_time += end_time.as_nanos();
            }
            let avg_time = total_time as f64 / num_tests as f64;
            println!(
                "Average time to hash a string of length {}: {} nanoseconds",
                string_length, avg_time
            );
        }
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
        let defalut_hash = |x: &str| -> u64 {
            let mut hasher = DefaultHasher::new();
            x.hash(&mut hasher);
            hasher.finish()
        };
        test_hash_function(defalut_hash, 10000, 100);
        test_hash_function(a_hash, 10000, 100);
        let murmur_hash = |x: &str| -> u64 { hash64(x.as_bytes()) };
        test_hash_function(murmur_hash, 10000, 100);
        test_hash_function(sea_hash, 10000, 100)
}
