use std::{num::Wrapping};

use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Key of generate hash for database/table
    #[arg(short, long)]
    key: String,

    /// Number of split target
    #[arg(short, long, default_value_t = 64)]
    count: i32,

    /// rehash base
    #[arg(short, long, default_value_t = 0)]
    rehash: i32,
}

fn main() {
    let args = Args::parse();
    let hash = java_hash(args.key.as_str());
    if args.rehash == 0 {
        println!("{:?}", (hash % args.count).abs());
    } else {
        println!("{:?}", ((hash / args.rehash) % args.count).abs());
    }
    
}

pub fn java_hash(key: &str) -> i32 {
    let mut hash = 0;
    let bytes = key.as_bytes();
    let wrapping_times: Wrapping<i32> = Wrapping(31);
    for v in bytes {
        let hash_wrapper: Wrapping<i32> = Wrapping(hash);
        hash = (wrapping_times * hash_wrapper).0 + (*v as i32);
    }
    return hash;
}
