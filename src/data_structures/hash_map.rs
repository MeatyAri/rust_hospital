use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

const INITIAL_CAPACITY: usize = 16;

#[derive(Debug, Serialize, Deserialize)]
pub struct HashMap<K: Clone, V: Clone> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); INITIAL_CAPACITY],
            size: 0,
        }
    }

    fn hash<Q: ?Sized>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        for &mut (ref existing_key, ref mut existing_value) in &mut self.buckets[index] {
            if existing_key == &key {
                *existing_value = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
        self.size += 1;
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .find(|&&(ref existing_key, _)| existing_key.borrow() == key)
            .map(|&(_, ref value)| value)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.hash(key);
        self.buckets[index]
            .iter_mut()
            .find(|&&mut (ref existing_key, _)| existing_key.borrow() == key)
            .map(|&mut (_, ref mut value)| value)
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];
        if let Some(pos) = bucket.iter().position(|&(ref existing_key, _)| existing_key.borrow() == key) {
            self.size -= 1;
            Some(bucket.swap_remove(pos).1)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .any(|&(ref existing_key, _)| existing_key.borrow() == key)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.buckets.iter_mut().flat_map(|bucket| {
            bucket.iter_mut().map(|&mut (_, ref mut value)| value)
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets.iter().flat_map(|bucket| {
            bucket.iter().map(|(key, value)| (key, value))
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut K, &mut V)> {
        self.buckets.iter_mut().flat_map(|bucket| {
            bucket.iter_mut().map(|(key, value)| (key, value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
        assert_eq!(map.get(&"key1"), Some(&"value1"));
        assert_eq!(map.get(&"key2"), Some(&"value2"));
        assert_eq!(map.get(&"key3"), None);
    }

    #[test]
    fn test_remove() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        assert_eq!(map.remove(&"key1"), Some("value1"));
        assert_eq!(map.get(&"key1"), None);
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut map = HashMap::new();
        assert!(map.is_empty());
        map.insert("key1", "value1");
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn test_update_value() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key1", "value2");
        assert_eq!(map.get(&"key1"), Some(&"value2"));
    }

    #[test]
    fn test_contains_key() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        assert!(map.contains_key(&"key1"));
        assert!(!map.contains_key(&"key2"));
    }

    #[test]
    fn test_get_mut() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        if let Some(value) = map.get_mut(&"key1") {
            *value = "value2";
        }
        assert_eq!(map.get(&"key1"), Some(&"value2"));
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        assert_eq!(map.remove(&"key1"), None);
    }

    #[test]
    fn test_insert_multiple_keys_with_same_hash() {
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key1", "value2");
        map.insert("key1", "value3");
        assert_eq!(map.get(&"key1"), Some(&"value3"));
    }
}