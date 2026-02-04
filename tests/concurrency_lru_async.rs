use std::sync::Arc;

use test_thread_safe_lru_cache::AsyncLruCache;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_async_concurrent_put_no_panic() {
    let cache = Arc::new(AsyncLruCache::new(100));
    let mut handles = Vec::new();

    for i in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(async move {
            for j in 0..1000 {
                cache
                    .put(format!("k{}", i * 1000 + j), format!("v{}", i * 1000 + j))
                    .await;
            }
        }));
    }

    for handle in handles {
        handle.await.expect("error while join the async handle");
    }

    cache.put("final".to_owned(), "ok".to_owned()).await;
    assert_eq!(cache.get(&"final".to_owned()).await, Some("ok".to_owned()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_async_concurrent_get_no_panic() {
    let cache = Arc::new(AsyncLruCache::new(10));

    for i in 0..10 {
        cache.put(format!("k{}", i), format!("v{}", i)).await;
    }

    let mut handles = Vec::new();

    for _ in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(async move {
            for i in 0..10 {
                let _ = cache.get(&format!("k{}", i)).await;
            }
        }));
    }

    for handle in handles {
        handle.await.expect("error while join the async handle");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_async_concurrent_mixed_read_write() {
    let cache = Arc::new(AsyncLruCache::new(100));

    cache.put("alien".to_owned(), "value".to_owned()).await;
    let mut handles = Vec::new();

    for i in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(async move {
            for j in 0..1000 {
                cache
                    .put(format!("k{}", i * 1000 + j), format!("v{}", i * 1000 + j))
                    .await;
                let _ = cache.get(&"alien".to_owned());
            }
        }));
    }

    for handle in handles {
        handle.await.expect("error while join the async handle");
    }

    cache.put("final".to_owned(), "ok".to_owned()).await;
    assert_eq!(cache.get(&"final".to_owned()).await, Some("ok".to_owned()));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_async_concurrent_cache_length_bounded() {
    let cache = Arc::new(AsyncLruCache::new(50));
    let mut handles = Vec::new();

    for i in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(tokio::spawn(async move {
            for j in 0..1000 {
                cache
                    .put(format!("k{}", i * 1000 + j), format!("v{}", j))
                    .await;
            }
        }));
    }

    for handle in handles {
        handle.await.expect("error while join the async handle");
    }

    assert!(cache.len().await <= 50);
}
