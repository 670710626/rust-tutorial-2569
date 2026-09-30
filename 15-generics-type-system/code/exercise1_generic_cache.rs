use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CacheEntry<K, V> {
    pub key: K,
    pub value: V,
    pub access_count: usize,
}

pub struct SimpleCache<K, V> {
    entries: Vec<CacheEntry<K, V>>,
    capacity: usize,
}

impl<K, V> SimpleCache<K, V>
where
    K: PartialEq + Display,
    V: Clone + Display,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        for entry in &mut self.entries {
            if entry.key == key {
                entry.value = value;
                entry.access_count += 1;
                return;
            }
        }

        if self.entries.len() >= self.capacity {
            self.entries.remove(0);
        }

        self.entries.push(CacheEntry {
            key,
            value,
            access_count: 1,
        });
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        for entry in &mut self.entries {
            if &entry.key == key {
                entry.access_count += 1;
                return Some(entry.value.clone());
            }
        }
        None
    }

    pub fn display_status(&self) {
        println!("=== Cache Status (Items: {}/{}) ===", self.entries.len(), self.capacity);
        for entry in &self.entries {
            println!("Key: '{}' => Value: '{}' (Accessed {} times)", 
                entry.key, entry.value, entry.access_count);
        }
        println!();
    }
}

fn main() {
    let mut session_cache = SimpleCache::new(3);

    session_cache.put("user_101".to_string(), 95);
    session_cache.put("user_102".to_string(), 80);
    session_cache.put("user_103".to_string(), 88);
    session_cache.display_status();

    println!("Get user_101: {:?}", session_cache.get(&"user_101".to_string()));
    println!("Get user_101 again: {:?}", session_cache.get(&"user_101".to_string()));
    session_cache.display_status();

    // ทดสอบ FIFO Eviction เมื่อเกินความจุ 3 รายการ
    session_cache.put("user_104".to_string(), 99);
    session_cache.display_status();

    match session_cache.get(&"user_101".to_string()) {
        Some(val) => println!("Found: {}", val),
        None => println!("'user_101' evicted as expected (None returned)."),
    }
}
