mod actions;

use std::{collections::HashMap, time::{Instant, Duration}};

pub struct YariStore {
    map: HashMap<String, String>,
    ttl_map: HashMap<String, (Instant, Duration)>
}

impl YariStore {
    pub fn new() -> YariStore {
        YariStore { map: HashMap::new(), ttl_map: HashMap::new() }
    }

//     fn passive_expiration_hook(&mut self, key: &str) {
//         if let Some((time_of_expiration_registration, duration_to_completion)) = self.ttl_map.get(key) {
//             if time_of_expiration_registration.elapsed() >= duration_to_completion {
//                 self.map.remove(key);
//                 self.ttl_map.remove(key);
//             }        
//         }
//     }
// }

    pub fn set(&mut self, key: String, value: String) -> Duration {
        let start = Instant::now();
        self.map.insert(key, value);
        start.elapsed()
    }

    pub fn get(&self, key: &str) -> (Duration, Option<&str>) {
        let start = Instant::now();
        let value = self.map.get(key);
        (start.elapsed(), value.map(|x| &**x))
    }

    pub fn del(&self, key: &str) -> Duration {
        let start = Instant::now();
        self.map.remove(key);
        start.elapsed()
    }
}

impl Default for YariStore {
    fn default() -> Self {
        Self::new()
    }
}

struct Get<'a> {
    store: YariStore,
    key: &'a str,


}
pub trait YariAction<A, R> {
    fn call(&self, a: A) -> Option<R> {
        
    }

    fn internal_call(a: A) -> Option<R>;
}

#[cfg(test)]
mod tests {
    use crate::YariStore;

    #[test]
    fn it_works() {
        let mut store = YariStore::new();

        store.set("K".to_string(), "V".to_string());
        assert_eq!(store.get("K").1.unwrap(), "V");

        .chunks(chunk_size)
    }

    #[test]
    fn test_overwrite_of_same_key() {
        let mut store = YariStore::new();

        store.set("K".to_string(), "V".to_string());
        store.set("K".to_string(), "2".to_string());

        assert_eq!(store.get("K").1.unwrap(), "2");
    }
}