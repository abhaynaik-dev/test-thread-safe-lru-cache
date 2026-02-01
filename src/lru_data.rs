use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LruCacheData<K, V> {
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>, // The nodes are always pushed, not removed
    head: Option<usize>,
    tail: Option<usize>,
    capacity: usize,
}

impl<K: Eq + Hash + Clone, V: Clone> LruCacheData<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            nodes: Vec::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let &index = self.map.get(key)?;
        self.move_to_front(index); // This does not write the data but reorder the structre 
        Some(&self.nodes[index].value)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }

        // Key is already present in the hash map
        if let Some(&index) = self.map.get(&key) {
            self.nodes[index].value = value;
            self.move_to_front(index);
            return;
        }

        if self.map.len() == self.capacity {
            self.remove_lru_node();
        }

        let index = self.nodes.len();
        self.nodes.push(Node {
            key: key.clone(),
            value,
            prev: None,
            next: None,
        });

        self.map.insert(key, index);
        self.insert_at_front(index);
    }

    pub fn len(&mut self) -> usize {
        self.map.len()
    }

    fn remove_lru_node(&mut self) {
        if let Some(lru_index) = self.tail {
            let key = self.nodes[lru_index].key.clone();
            self.remove_node(lru_index);
            self.map.remove(&key);
        }
    }

    fn insert_at_front(&mut self, index: usize) {
        self.nodes[index].prev = None;
        self.nodes[index].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(index);
        }

        self.head = Some(index);

        if self.tail.is_none() {
            self.tail = Some(index);
        }
    }

    fn move_to_front(&mut self, index: usize) {
        if Some(index) == self.head {
            return;
        }
        self.remove_node(index);
        self.insert_at_front(index);
    }

    fn remove_node(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;

        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }

        if let Some(n) = next {
            self.nodes[n].prev = prev;
        } else {
            self.tail = prev;
        }

        self.nodes[index].prev = None;
        self.nodes[index].next = None;
    }
}

#[cfg(test)]
mod tests {
    use super::LruCacheData;

    #[test]
    fn test_basic_put_get() {
        let mut cache = LruCacheData::new(2);
        cache.put("k1", "v1");
        cache.put("k2", "v2");

        assert_eq!(cache.get(&"k1"), Some(&"v1"));
        assert_eq!(cache.get(&"k2"), Some(&"v2"));
    }

    #[test]
    fn test_remove_lru() {
        let mut cache = LruCacheData::new(2);
        cache.put("k1", "v1");
        cache.put("k2", "v2");
        cache.put("k3", "v3");

        assert_eq!(cache.get(&"k1"), None);
        assert_eq!(cache.get(&"k2"), Some(&"v2"));
        assert_eq!(cache.get(&"k3"), Some(&"v3"));
    }
}
