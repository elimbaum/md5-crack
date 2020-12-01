use md5::{Md5, Digest};
use std::env;
use std::iter;
use std::process;
use std::time::Instant;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

//// tests ////
// abc -> 900150983cd24fb0d6963f7d28e17f72
// dog -> 06d80eb0c50b49a509b49f2424e8c805
// yelp -> 771c0159ac754c62cdc1c5981d1412f9
// free -> aa2d6e4f578eb0cfaba23beef76c2194
// fred -> 570a90bfbf8c7eab5dc5d4e26832d5b1
// apple -> 1f3870be274f6c49b3e31a0c6728957f
// hOrSe -> 55ec8e764ce197dac6c7ae83dbf1c5bf
// snake -> de1b2a7baf7850243db71c4abd4e5a39
// pandas -> 3a43b4f88325d94022c0efa9c2fa2f5a
// password -> 5f4dcc3b5aa765d61d8327deb882cf99

// sequential: ~2M/sec
// parallel (but with other programs open):

// permutations: arrangements of elements
// combinations: no repetition
// combinations w/ replacement: order doesn't matter!
// permutations w/ replacement, or cartesian product: replacement and order matters

/*
TODO
  - give each thread its own comparison hash
  - try with actual threads, real speedup?
  - figure out order of iterator processing
  - thread local counters; add at end
  - don't use strings - the back and forth is slow
*/

const _ASCIIALNUM : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const _ASCIILOWER : &str = "abcdefghijklmnopqrstuvwxyz";
const _ASCIIALNUM_PUNC : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ ";

const ASCIICHARS : &str = _ASCIIALNUM;

fn hex_str_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                      .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}

fn main() {
    let hash = match env::args().nth(1) {
        Some(arg) => arg,
        None => { println!("no hash!"); process::exit(1) },
    };

    // let hash = "1f3870be274f6c49b3e31a0c6728957f";

    let hash_bytes = match hex_str_to_bytes(&hash) {
        Some(bytes) => bytes,
        None => { println!("invalid hash!"); process::exit(1) },
    };

    println!("cracking {} against {}-char set", hash, ASCIICHARS.len());

    // create a Md5 hasher instance
    // let mut hasher = Md5::new();

    let atm_hash_count = AtomicU64::new(0);

    let now = Instant::now();

    let mut len = 1;
    let mut result = None;
    while let None = result {
        // let thread_hash_counts = Arc::new(ThreadLocal::new());

        // i need cartesian product here, not perms
        println!("Trying {}-len strings", len);
        result = iter::repeat(ASCIICHARS.chars())
                    .take(len)
                    .multi_cartesian_product()
                    .par_bridge()
                    .map(|x| x.into_iter().collect::<String>())
                    .find_any(|xs|
        {
          // let xs = x.into_iter().collect::<String>();
          // println!("{}", xs);

          // if no count in this thread, make a new one; else increment
          // let count = thread_hash_counts.get_or(|| Cell::new(0));
          // count.set(count.get() + 1);

          let mut hasher = Md5::new();
          hasher.update(&xs);
          let result = hasher.finalize();

          atm_hash_count.fetch_add(1, Ordering::Relaxed);

          return result.iter().zip(hash_bytes.iter()).all(|(&a, &b)| a == b);
        });
        len += 1;

        // let thread_hash_counts = Arc::try_unwrap(thread_hash_counts).unwrap();
        // main_hash_count += thread_hash_counts.into_iter().fold(0, |x, y| x + y.get());
    }

    let elapsed_ms = now.elapsed().as_millis();

    // println!("{}", result.unwrap().into_iter().collect::<String>());
    println!("{}", result.unwrap());

    let hash_count = atm_hash_count.into_inner();

    println!("{} hashes", hash_count);
    println!("{} ms", elapsed_ms);
    println!("{} hash/s", hash_count * 1000 / elapsed_ms as u64)

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    // let result = hasher.finalize();
    // println!("{:x?}", result);
    // assert_eq!(result[..].to_vec(), hash_bytes);
}
