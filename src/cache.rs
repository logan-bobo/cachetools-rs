pub trait Cache<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&mut self, key: &K) -> Option<&V>;
    fn peek(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn contains_key(&self, key: &K) -> bool {
        self.peek(key).is_some()
    }
    // TODO: Add a drain function here to return all cached
    // keys and values and clear the cache
}
