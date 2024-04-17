use std::hash::{self, Hasher, DefaultHasher};

pub fn calculate_hash<T: hash::Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
