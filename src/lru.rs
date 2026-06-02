use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::cache::Cache;

pub struct LruCache<K, V> {
    data: HashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K, V> LruCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(capacity: usize) -> LruCache<K, V> {
        LruCache {
            data: HashMap::new(),
            order: VecDeque::new(),
            capacity: capacity,
        }
    }

    fn touch(&mut self, key: &K) {
        if let Some(index) = self.order.iter().position(|k| k == key) {
            self.order.remove(index);
            self.order.push_back(key.clone());
        }
    }
}
