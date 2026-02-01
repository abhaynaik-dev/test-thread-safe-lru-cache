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
        let mut guard = self.data.lock().unwrap();
        guard.get(key).cloned()
    }

    pub fn put(&self, key: K, value: V) {
        let mut guard = self.data.lock().unwrap();
        guard.put(key, value);
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
}
