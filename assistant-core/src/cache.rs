use lru::LruCache;
use once_cell::sync::Lazy;
use std::num::NonZeroUsize;
use std::sync::Mutex;

static CACHE: Lazy<Mutex<LruCache<String, String>>> = Lazy::new(|| {
    Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap()))
});

pub fn get_cached(key: &str) -> Option<String> {
    let mut c = CACHE.lock().unwrap();
    c.get(key).cloned()
}

pub fn set_cached(key: &str, value: String) {
    let mut c = CACHE.lock().unwrap();
    c.put(key.to_string(), value);
}
