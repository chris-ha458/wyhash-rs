// Example using the std::hash::Hasher interface.

extern crate wyhash;
use wyhash::WyHash;
use std::hash::Hasher;

fn main() {
    let mut hasher = WyHash::with_seed(1);
    hasher.write(&[0]);
    assert_eq!(0xcb4b8ebdf7240e2c, hasher.finish());
}
