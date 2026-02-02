use test_thread_safe_lru_cache::LruCache;

use std::sync::Arc;
use std::thread;
use std::time::Instant;

#[test]
fn test_concurrent_put_no_panic() {
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

#[test]
#[ignore]
fn test_heavy_stress_mixed_contention() {
    let threads = 16;
    let cache = Arc::new(LruCache::new(1000));

    let start = Instant::now();
    let mut handles = Vec::new();

    for t in 0..threads {
        let cache = Arc::clone(&cache);

        let key = "k1".to_string();
        handles.push(thread::spawn(move || {
            for i in 0..100000 {
                cache.put(format!("k{}", t * i), format!("v{}", t * i));
                let _ = cache.get(&key);
            }
        }));
    }

    for handle in handles {
        handle.join().expect("error while join the handle");
    }

    println!(
        "heavy workload mixed contention elapsed: {:?}",
        start.elapsed()
    );
}
