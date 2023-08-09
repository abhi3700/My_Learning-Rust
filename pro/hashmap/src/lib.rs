use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use std::mem;

const INITIAL_BUCKET_SIZE: u32 = 1;

/// A bucket contains a list of (K, V) pairs.
///
// struct Bucket<K, V> {
//     items: Vec<(K, V)>,
// }

/// Cons: not so memory efficient
/// Every key has a bucket dedicated, hence the lookup time is not going to increase because,
/// there is just element in the bucket [].
/// [(,)] <- [(,)] <- [(,)]
/// A HashMap is a list of Bucket
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

// NOTE: Don't implement this because it is a more simple implementation of vector of key, value pair
// Cons: lookup time is going to increase to find a key.
// Pros: memory efficient.
// (,) <- (,) <- (,)
// pub struct HashMap2<K, V> {
//     buckets: Vec<(K, V)>,
// }

impl<K, V> HashMap<K, V> {
    fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + std::clone::Clone,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Here, the capacity of the HashMap is to be increased not every time.....whenever
        // empty & when reaching close to capacity of hashmap
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        // get the index
        let bucket: usize = hasher.finish() as usize % self.buckets.len();
        // get the bucket with hashed key
        let bucket = &mut self.buckets[bucket];

        // match the key, if found => replace the value with new value & return Some()
        for (ekey, evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }

        self.items += 1; // increase the len. TODO: decide the placing inside the function

        // if key not matched, push the new (k, v) into the buckets.
        bucket.push((key, value));

        None
    }

    // increasing the capacity
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKET_SIZE as usize,
            n => 2 * n,
        };

        // drain the existing buckets & allocate to a new bucket.
        // Cons: expensive process

        let mut new_buckets = Vec::<Vec<(K, V)>>::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);

            // get the index
            let bucket: usize = hasher.finish() as usize % new_buckets.len();

            new_buckets[bucket].push((key, value));
        }

        let _ = mem::replace(&mut self.buckets, new_buckets);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hashmap_works() {
        let mut hm1 = HashMap::<&str, i32>::new();

        hm1.insert("foo", 42);
        // assert_eq!(hm1.)
    }
}
