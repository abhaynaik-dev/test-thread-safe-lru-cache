use std::hash::Hash;
use tokio::sync::Mutex;

use crate::lru_data::LruCacheData;

pub struct AsyncLruCache<K, V> {
    data: Mutex<LruCacheData<K, V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> AsyncLruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Mutex::new(LruCacheData::new(capacity)),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let mut guard = self.data.lock().await;
        guard.get(key).cloned()
    }

    pub async fn put(&self, key: K, value: V) {
        let mut guard = self.data.lock().await;
        guard.put(key, value);
    }

    pub async fn len(&self) -> usize {
        let guard = self.data.lock().await;
        guard.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncLruCache;

    #[tokio::test]
    async fn test_async_basic_put_get() {
        let cache = AsyncLruCache::new(2);
        cache.put("k1", "v1").await;
        cache.put("k2", "v2").await;

        assert_eq!(cache.get(&"k1").await, Some("v1"));
        assert_eq!(cache.get(&"k2").await, Some("v2"));
    }

    #[tokio::test]
    async fn test_async_zero_capacity_cache() {
        let cache = AsyncLruCache::new(0);
        cache.put("k1", "v1").await;

        assert_eq!(cache.get(&"k1").await, None);
    }

    #[tokio::test]
    async fn test_async_remove_lru() {
        let cache = AsyncLruCache::new(2);
        cache.put("k1", "v1").await;
        cache.put("k2", "v2").await;
        cache.put("k3", "v3").await;

        assert_eq!(cache.get(&"k1").await, None);
        assert_eq!(cache.get(&"k2").await, Some("v2"));
        assert_eq!(cache.get(&"k3").await, Some("v3"));
    }

    #[tokio::test]
    async fn test_async_udpate_lru() {
        let cache = AsyncLruCache::new(2);
        cache.put("k1", "v1").await;
        cache.put("k2", "v2").await;

        // Update the existing key making MRU
        cache.put("k1", "v11").await;

        // Add new key, should remove k2
        cache.put("k3", "v3").await;

        assert_eq!(cache.get(&"k1").await, Some("v11"));
        assert_eq!(cache.get(&"k2").await, None);
        assert_eq!(cache.get(&"k3").await, Some("v3"));
    }

    #[tokio::test]
    async fn test_async_cache_length_bounded() {
        let cache = AsyncLruCache::new(3);

        cache.put("k1".to_string(), "v1".to_string()).await;
        cache.put("k2".to_string(), "v2".to_string()).await;
        cache.put("k3".to_string(), "v3".to_string()).await;
        cache.put("k4".to_string(), "v4".to_string()).await;
        cache.put("k5".to_string(), "v5".to_string()).await;

        assert!(cache.len().await <= 3);
    }
}
