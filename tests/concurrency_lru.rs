use test_thread_safe_lru_cache::LruCache;

#[test]
fn test_concurrent_put_no_panic() {
    use std::sync::Arc;
    use std::thread;

    let cache = Arc::new(LruCache::new(100));

    let mut handles = Vec::new();

    for i in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for j in 0..1000 {
                cache.put(format!("k{}", i * 1000 + j), format!("v{}", i * 1000 + j));
            }
        }));
    }

    for handle in handles {
        handle.join().expect("error while join the handle");
    }

    cache.put("final".to_owned(), "ok".to_owned());
    assert_eq!(cache.get(&"final".to_owned()), Some("ok".to_owned()));
}

#[test]
fn test_concurrent_get_no_panic() {
    use std::sync::Arc;
    use std::thread;

    let cache = Arc::new(LruCache::new(10));

    for i in 0..10 {
        cache.put(format!("k{}", i), format!("v{}", i));
    }

    let mut handles = Vec::new();

    for _ in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for i in 0..10 {
                let _ = cache.get(&format!("k{}", i));
            }
        }));
    }

    for handle in handles {
        handle.join().expect("error while join the handle");
    }
}

#[test]
fn test_concurrent_mixed_read_write() {
    use std::sync::Arc;
    use std::thread;

    let cache = Arc::new(LruCache::new(100));

    cache.put("alien".to_owned(), "value".to_owned());

    let mut handles = Vec::new();

    for i in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for j in 0..1000 {
                cache.put(format!("k{}", i * 1000 + j), format!("v{}", i * 1000 + j));
                let _ = cache.get(&"alien".to_owned());
            }
        }));
    }

    for handle in handles {
        handle.join().expect("error while joining thread");
    }

    cache.put("final".to_owned(), "ok".to_owned());
    assert_eq!(cache.get(&"final".to_owned()), Some("ok".to_owned()));
}

#[test]
fn test_concurrent_cache_length_bounded() {
    use std::sync::Arc;
    use std::thread;

    let cache = Arc::new(LruCache::new(100));
    let mut handles = Vec::new();

    for i in 0..8 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for j in 0..1000 {
                cache.put(format!("k{}", i * 1000 + j), format!("v{}", j));
            }
        }));
    }

    for handle in handles {
        handle.join().expect("error while join the handle");
    }

    assert!(cache.len() <= 100);
}
