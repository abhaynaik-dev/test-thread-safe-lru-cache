use std::hash::Hash;
use std::sync::Mutex;

use crate::lru_data::LruCacheData;

pub struct LruCache<K, V> {
    data: Mutex<LruCacheData<K, V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Mutex::new(LruCacheData::new(capacity)),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut guard = self
            .data
            .lock()
            .expect("failed to acquire get LruCache lock");
        guard.get(key).cloned()
    }

    pub fn put(&self, key: K, value: V) {
        let mut guard = self
            .data
            .lock()
            .expect("failed to acquire post LruCache lock");
        guard.put(key, value);
    }

    pub fn len(&self) -> usize {
        let mut guard = self
            .data
            .lock()
            .expect("failed to acquire get LruCache lock");
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn test_basic_put_get() {
        let cache = LruCache::new(2);
        cache.put("k1", "v1");
        cache.put("k2", "v2");

        assert_eq!(cache.get(&"k1"), Some("v1"));
        assert_eq!(cache.get(&"k2"), Some("v2"));
    }

    #[test]
    fn test_zero_capacity_cache() {
        let cache = LruCache::new(0);
        cache.put("k1", "v1");

        assert_eq!(cache.get(&"k1"), None);
    }

    #[test]
    fn test_remove_lru() {
        let cache = LruCache::new(2);
        cache.put("k1", "v1");
        cache.put("k2", "v2");
        cache.put("k3", "v3");

        assert_eq!(cache.get(&"k1"), None);
        assert_eq!(cache.get(&"k2"), Some("v2"));
        assert_eq!(cache.get(&"k3"), Some("v3"));
    }

    #[test]
    fn test_udpate_lru() {
        let cache = LruCache::new(2);
        cache.put("k1", "v1");
        cache.put("k2", "v2");

        // Update the existing key making MRU
        cache.put("k1", "v11");

        // Add new key, should remove k2
        cache.put("k3", "v3");

        assert_eq!(cache.get(&"k1"), Some("v11"));
        assert_eq!(cache.get(&"k2"), None);
        assert_eq!(cache.get(&"k3"), Some("v3"));
    }

    #[test]
    fn test_cache_length_bounded() {
        let cache = LruCache::new(3);

        cache.put("k1".to_string(), "v1".to_string());
        cache.put("k2".to_string(), "v2".to_string());
        cache.put("k3".to_string(), "v3".to_string());
        cache.put("k4".to_string(), "v4".to_string());
        cache.put("k5".to_string(), "v5".to_string());

        assert!(cache.len() <= 3);
    }
}
