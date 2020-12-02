#![allow(unused_imports)]
use md5::{Md5, Digest};
use std::env;
use std::iter;
use std::str;
use std::process;
use std::borrow::BorrowMut;
use std::time::Instant;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::cell::{RefCell, Cell};
use thread_local::CachedThreadLocal;

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

    // let atm_hash_count = AtomicU64::new(0);
    let mut main_hash_count = 0;

    let now = Instant::now();

    // convert the source char set into bytes
    // WARNING: THIS ONLY WORKS FOR ASCII (i.e. 1-byte chars)
    let char_set = Vec::from(ASCIICHARS.as_bytes());

    let mut len = 1;
    let mut result = None;
    while let None = result {
        let thread_hashers = CachedThreadLocal::new();
        let thread_counts = CachedThreadLocal::new();

        println!("Trying {}-len strings", len);
        result = iter::repeat(char_set.clone())
                    .take(len)
                    .multi_cartesian_product()
                    // .par_bridge()
                    .find(|xs|
        {

          let mut hasher = thread_hashers.get_or(|| RefCell::new(Md5::new())).borrow_mut();
          hasher.update(&xs);
          let result = hasher.finalize_reset();

          let count = thread_counts.get_or(|| Cell::new(0));
          count.set(count.get() + 1);

          return result.iter().zip(hash_bytes.iter()).all(|(&a, &b)| a == b);
        });
        len += 1;

        main_hash_count += thread_counts.into_iter()
                                        // .inspect(|x| println!("{:?}", x))
                                        .fold(0, |x, y| x + y.get());
    }

    let elapsed_ms = now.elapsed().as_millis();

    // println!("{}", result.unwrap().into_iter().collect::<String>());
    println!("{}", String::from_utf8(result.unwrap()).unwrap());

    // let hash_count = atm_hash_count.into_inner();

    println!("{} hashes", main_hash_count);
    println!("{} ms", elapsed_ms);
    println!("{} hash/s", main_hash_count * 1000 / elapsed_ms as u64)

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    // let result = hasher.finalize();
    // println!("{:x?}", result);
    // assert_eq!(result[..].to_vec(), hash_bytes);
}
