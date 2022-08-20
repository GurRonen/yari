use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

trait YariKeyspace {
    fn yari_get(&self, key: &str) -> Option<&str>;
    fn yari_set(&mut self, key: String, value: String) -> Option<String>;
    fn yari_del(&mut self, key: &str) -> usize;

    fn yari_expire(&mut self, key: &str, until: Duration) -> bool {
        self.yari_expire_since_instant(key, Instant::now(), until)
    }
    fn yari_expire_since_instant(&mut self, key: &str, from: Instant, until: Duration) -> bool;

    fn yari_lcontains(&self, key: &str) -> Option<bool>;
    fn yari_lpush(&mut self, key: &str, value: String) -> usize;
    fn yari_lrange(&self, key: &str, start_index: i32, end_index: i32) -> &[&str];
    fn yari_lindex(&self, index: i32) -> &str;

    fn yari_optimize_memory(&mut self);
}

struct YariHashMapKeyspace {
    map: HashMap<String, String>,
    ttls: HashMap<String, Instant>,
}

#[allow(unused_variables)]
impl YariKeyspace for YariHashMapKeyspace {
    fn yari_get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|x| &x[..])
    }

    fn yari_set(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }

    fn yari_del(&mut self, key: &str) -> usize {
        self.map.remove(key).map_or(0, |_| 1)
    
    }
    fn yari_expire_since_instant(&mut self, key: &str, from: Instant, until: Duration) -> bool {
        self.ttls.insert(String::from(key), from + until);
        true
    }

    fn yari_lcontains(&self, key: &str) -> Option<bool> {
        todo!()
    }

    fn yari_lpush(&mut self, key: &str, value: String) -> usize {
        todo!()
    }

    fn yari_lrange(&self, key: &str, start_index: i32, end_index: i32) -> &[&str] {
        todo!()
    }

    fn yari_lindex(&self, index: i32) -> &str {
        todo!()
    }

    fn yari_optimize_memory(&mut self) {
        self.map.shrink_to_fit();
        self.ttls.shrink_to_fit();
    }
}
