use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use rand::seq::IteratorRandom;

use bytes::Bytes;


pub(crate) trait YariKeyspace {
    fn yari_get(&mut self, key: &str) -> Option<&Bytes>;
    fn yari_set(&mut self, key: String, value: Bytes) -> Option<Bytes>;
    fn yari_del(&mut self, key: &str) -> usize;

    fn yari_expire(&mut self, key: &str, until: Duration) -> bool {
        self.yari_expire_since_instant(key, Instant::now(), until)
    }
    fn yari_expire_since_instant(&mut self, key: &str, from: Instant, until: Duration) -> bool;

    fn yari_lcontains(&self, key: &str) -> Option<bool>;
    fn yari_lpush(&mut self, key: &str, value: Bytes) -> usize;
    fn yari_lrange(&self, key: &str, start_index: i32, end_index: i32) -> &[&Bytes];
    fn yari_lindex(&self, index: i32) -> &Bytes;

    fn yari_optimize_memory(&mut self);
    fn yari_active_expiration(&mut self);
}

pub(crate) struct YariHashMapKeyspace {
    map: HashMap<String, Bytes>,
    ttls: HashMap<String, Instant>,

    rerun_expiration_threshold_percentage: f32,
    amount_of_keys_for_active_expiration: usize
}

impl Default for YariHashMapKeyspace {
    fn default() -> Self {
        Self { map: Default::default(), ttls: Default::default(), rerun_expiration_threshold_percentage: 0.25, amount_of_keys_for_active_expiration: 20 }
    }
}

impl YariHashMapKeyspace {
    pub(crate) fn new(initial_capacity: usize, rerun_expiration_threshold_percentage: f32, amount_of_keys_for_active_expiration: usize) -> YariHashMapKeyspace {
        YariHashMapKeyspace {
            map: HashMap::with_capacity(initial_capacity),
            ttls: HashMap::with_capacity(initial_capacity),
            rerun_expiration_threshold_percentage,
            amount_of_keys_for_active_expiration,
        }
    }
    
    fn expire_specific_key(&mut self, key: &str) {
        self.map.remove(key);
        self.ttls.remove(key);
    }
}

#[allow(unused_variables)]
impl YariKeyspace for YariHashMapKeyspace {
    fn yari_get(&mut self, key: &str) -> Option<&Bytes> {
        if let Some(value) = self.ttls.get(key) {
            if value < &Instant::now() {
                self.expire_specific_key(key);
                return None;
            }
        }
        self.map.get(key)
    }

    fn yari_set(&mut self, key: String, value: Bytes) -> Option<Bytes> {
        self.map.insert(key, value)
    }

    fn yari_del(&mut self, key: &str) -> usize {
        self.ttls.remove(key);
        self.map.remove(key).map_or(0, |_| 1)
    }
    fn yari_expire_since_instant(&mut self, key: &str, from: Instant, until: Duration) -> bool {
        self.ttls.insert(String::from(key), from + until);
        true
    }

    fn yari_lcontains(&self, key: &str) -> Option<bool> {
        todo!()
    }

    fn yari_lpush(&mut self, key: &str, value: Bytes) -> usize {
        todo!()
    }

    fn yari_lrange(&self, key: &str, start_index: i32, end_index: i32) -> &[&Bytes] {
        todo!()
    }

    fn yari_lindex(&self, index: i32) -> &Bytes {
        todo!()
    }

    fn yari_optimize_memory(&mut self) {
        self.map.shrink_to_fit();
        self.ttls.shrink_to_fit();
    }

    fn yari_active_expiration(&mut self) {

        let sample: Vec<(&String, &Instant)> = self.ttls
            .iter()
            .choose_multiple(&mut rand::thread_rng(), self.amount_of_keys_for_active_expiration);
            
        let sample_actual_len = sample.len();

        let removables: Vec<String> = sample.iter()
            .filter(|(a, b)| **b <= Instant::now())
            .map(|(key, instant)| (**key).clone())
            .collect();

        let removables_count = removables.len();
        for a in removables {
                self.expire_specific_key(&a);
        }
        if removables_count as f32 / sample_actual_len as f32 > self.rerun_expiration_threshold_percentage {
            self.yari_active_expiration();
        }

    }
}

#[cfg(test)]
mod tests {
    use std::{time::Duration, thread::sleep};

    use super::*;

    #[test]
    fn test_empty_map_is_empty() {
        let mut db = YariHashMapKeyspace::default();
        assert!(db.yari_get("a").is_none());
    }

    #[test]
    fn test_single_insert_single_read() {
        let mut db = YariHashMapKeyspace::default();
        db.yari_set("a".to_string(), Bytes::from("1"));

        assert_eq!(Bytes::from("1"), db.yari_get("a").unwrap())
    }

    #[test]
    fn test_two_inserts_single_read() {
        let mut db = YariHashMapKeyspace::default();
        db.yari_set("a".to_string(), Bytes::from("1"));
        db.yari_set("a".to_string(), Bytes::from("2"));

        assert_eq!(Bytes::from("2"), db.yari_get("a").unwrap())
    }

    #[test]
    fn test_simple_expiration() {
        let mut db = YariHashMapKeyspace::default();
        db.yari_set("a".to_string(), Bytes::from("1"));

        db.yari_expire("a", Duration::from_millis(1));
        sleep(Duration::from_millis(2));

        assert!(db.yari_get("a").is_none())
    }

    #[test]
    fn test_active_expiration() {
        let mut db = YariHashMapKeyspace::new(30, 0.1, 30);

        for a in 0..30 {
            db.yari_set(a.to_string(), Bytes::new());
        }

        db.yari_expire("0", Duration::ZERO);
        db.yari_expire("1", Duration::ZERO);

        assert_eq!(db.ttls.len(), 2);
        db.yari_active_expiration();
        assert_eq!(db.ttls.len(), 0);
    }

    #[test]
    fn test_active_expiration_with_2_milli() {
        let mut db = YariHashMapKeyspace::new(30, 1.0, 30);
        
        for a in 0..30 {
            db.yari_set(a.to_string(), Bytes::new());
        }

        db.yari_expire_since_instant("0", Instant::now() - Duration::from_secs(10), Duration::from_secs(11));
        db.yari_expire_since_instant("1", Instant::now() - Duration::from_secs(10), Duration::from_secs(9));

        assert!(db.yari_get("0").is_some());
        assert!(db.yari_get("1").is_none());

    }
}
