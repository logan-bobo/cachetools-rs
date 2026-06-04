use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::cache::Cache;
use crate::errors::CacheError;

// FIFOCache implements [Cache]
// First In First Out (FIFO) caching policy evicts the
// evicts keys in the order they were inserted
pub struct FIFOCache<K, V> {
    data: HashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K, V> FIFOCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn try_new(capacity: usize) -> Result<FIFOCache<K, V>, CacheError> {
        if capacity == 0 {
            return Err(CacheError::InvalidCacheCapacity);
        }

        Ok(FIFOCache {
            data: HashMap::new(),
            order: VecDeque::new(),
            capacity: capacity,
        })
    }
}

impl<K, V> Cache<K, V> for FIFOCache<K, V>
where
    K: Eq + Hash + Clone,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // In FIFO a updated to a value does not
        // change cache order
        if self.contains_key(&key) {
            return self.data.insert(key, value);
        }

        if self.order.len() >= self.capacity {
            self.order.pop_front().inspect(|key| {
                self.data.remove(&key);
            });
        };

        self.order.push_back(key.clone());
        self.data.insert(key, value)
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(index) = self.order.iter().position(|k| k == key) {
            self.order.remove(index);
        };
        self.data.remove(key)
    }

    fn clear(&mut self) {
        self.order.clear();
        self.data.clear();
    }

    fn peek(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.order.len()
    }

    fn is_empty(&self) -> bool {
        self.order.len() < 1
    }

    fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
}
