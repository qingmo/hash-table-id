use std::{num::Wrapping};

use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    key: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 64)]
    count: i32,
}

fn main() {
    let args = Args::parse();
    let hash = java_hash(args.key.as_str());
    println!("{:?}", hash.abs() % args.count);
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
