use std::{collections::HashMap, time::Duration};

enum YariValue {
    STR(String),
    LISTSTR(Vec<String>)

}

trait YariKeyspace {
    fn get(&self, key: &str) -> Option<&str>;
    fn set(&mut self, key: String, value: String) -> Option<String>;
    fn expire(&mut self, key: &str, ttl: Duration) -> bool;
    fn lcontains(&self, key: &str) -> Option<bool>;
    fn lpush(&mut self, key: &str, value: String) -> usize;
    fn lrange(&self, key: &str, start_index: i32, end_index: i32) -> &[&str];
    fn lindex(&self, index: i32) -> &str;
    
}

pub struct KAnyStore {
    map: HashMap<String, String>
}

/* pub struct KAnyStoreLogImpl {

} */

impl KAnyStore {
    pub fn get(&self) -> Get {
        todo!()
    }

    pub fn set(&self) {
        todo!()
    }

    pub fn expire(&self) {
        todo!()
    }
}


struct Get<'a, T: 'a + KAnyStore> {
    store: T,
    key: &str
}


struct Set<'a, T: 'a + KAnyStore, V> {
    store: T,
    key: &str,
    value: V
}

struct Expire<'a, T: 'a + KAnyStore> {
    store: T,
    key: &str,
    milliseconds: usize
}

