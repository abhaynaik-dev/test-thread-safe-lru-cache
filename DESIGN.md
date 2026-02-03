# Thread-Safe LRU Cache – Design

## Overview

This project implements a thread-safe Least Recently Used (LRU) cache in Rust.  
The cache supports concurrent access, provides O(1) average-time `get` and `put` operations, and removes the least recently used entry when the configured capacity is exceeded.

The detailed functional requirements are described in `README.md`.

## Data Structures

The cache is built using two core data structures:

### HashMap<K, usize>

A hash map is used to map each key to an index in the node storage vector.

- key: cache key (`K`)
- nodes: index of the corresponding node inside `Vec<Node<K, V>>`

This enables O(1) average-time lookup for both `get` and `put`.

### Vec<Node<K, V>>

Nodes are stored inside a vector, where each node represents a cache entry.

Each node contains:

- key: cache key (`K`)  
- value: cache value  
- prev index: stores index of previous node  
- next index: stores index of next node    

The linked list formed by these indices maintains recency ordering.

**Note:**  
Nodes are not removed from the vector when evicted from the LRU. Instead, their entries are removed from the hash map and their links are detached. This keeps eviction O(1) and avoids costly vector element shifting. Logical cache size remains bounded by capacity.

## Synchronization Strategy

### Sync Compatible Cache

The public cache is protected by a single `std::sync::Mutex` guarding the internal `LruCacheData`.

Both `get()` and `put()` acquire the lock.

### get(key)

1. Acquire lock  
2. Look up key in hash map  
   - If not found, return `None`  
   - If found, move node to front, MRU  
   - Return value  

Although `get()` appears to be a read operation, it mutates internal state by updating recency ordering. Therefore, it requires exclusive access.

### put(key, value)

1. Acquire lock  
2. If key exists, update value and move node to front  
3. If key does not exist:
   - If capacity reached, evict LRU entry  
   - Insert new node  
   - Add entry to map  
   - Insert node at front, MRU  

Using a Mutex keeps the implementation simple and ensures correctness under contention.

### Async Compatible Cache

In addition to the synchronous cache, an async compatible version is added.

The public cache is protected by a single `tokio::sync::Mutex` to guard the same internal `LruCacheData`.

The internal data structure and eviction logic are identical to the synchronous version. The only difference is that the lock acquisition is performed by `.lock().await` instead of blocking the OS thread. This ensures that other async tasks can continue running while waiting for the lock.

## LRU Ordering Under Concurrency

LRU ordering is maintained by a doubly linked list implemented using node indices.

- Head → Most Recently Used (MRU)  
- Tail → Least Recently Used (LRU)  

All operations that modify the list occur while holding the mutex. This ensures that only one thread can update the list structure at a time.

As a result, the cache maintains correct LRU semantics even under concurrent access. 

## Trade-offs

### Mutex vs RwLock

A `RwLock` was considered as an alternative to `Mutex`. However, in an LRU cache, `get()` is not a pure read operation because it updates recency ordering by moving the recently accessed entry to the front of the list. As a result, both `get()` and `put()` require exclusive access.

Because most operations require mutation, a `RwLock` would still spend most of its time acquiring a write lock, while also adding complexity and overhead.

It is possible to build a reader based design where `get()` first acquires a read lock to fetch the value and later acquires a write lock to update recency. However, this introduces a window where a key could be evicted between the two phases, resulting in a stale value being returned. Also this will have a performance impact due to extra lock acqusition.

For these reasons, a simple `Mutex` was chosen for predictable behavior with very minimal impact on performance.
This implementation prioritizes **strict LRU semantics and correctness** 

### Arc vs Rc / RefCell / Cell

The public cache is shared across threads, so it must support thread-safe shared ownership.

- `Arc<T>` provides atomic reference counting and is safe to use across threads  
- `Rc<T>` is not thread-safe and cannot be shared between threads  
- `RefCell<T>` and `Cell<T>` provide interior mutability but are not thread-safe  

Because the cache must be accessed concurrently by multiple threads, `Arc` combined with `Mutex` is the correct choice.

### Simplicity vs Maximum Scalability

This design favors a simple, correct baseline implementation with clear synchronization. More advanced techniques such as sharding or approximate eviction policies can improve scalability but add significant complexity.

## Performance Considerations

Informal stress tests were executed using 16 worker threads performing mixed read and write operations.
-  Mutex-based implementation: ~5.4 seconds
-  RwLock-based implementation: ~5.2 seconds

This shows almost similar performance for both approaches. 

## Known Limitations

- The cache uses a single global lock. Under heavy concurrent write workloads, this lock can become a bottleneck.
- Nodes are not reused inside the internal vector; memory usage remains bounded logically but indices may grow over time.
- Only Least Recently Used (LRU) eviction is supported.

## Possible Future Improvements

- Introducing node reuse or slab allocation for tighter memory management  
